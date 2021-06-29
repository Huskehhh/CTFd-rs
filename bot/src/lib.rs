#[macro_use]
extern crate failure;

use ctfdb::{ChallengeProvider, ctfd::db::{CTFD_CACHE, check_for_new_solves, get_active_ctfs, get_and_store_scoreboard, mark_solved, update_challenges_and_scores}, models::Challenge};
use failure::Error;
use serenity::{
    builder::CreateEmbed, framework::standard::CommandResult, http::Http, model::id::ChannelId,
};

pub type ChallengeProviderService = Box<dyn ChallengeProvider + Send + Sync>;

pub fn populate_embed_from_challenge(challenge: Challenge, e: &mut CreateEmbed) {
    e.title(format!("❓ {} ❓", challenge.name));
    e.field("📚 Category", &challenge.category, true);
    e.field("💰 Points", challenge.points, true);

    if challenge.working.is_some() {
        e.field("🧰 Working", challenge.working.unwrap(), true);
    }

    if challenge.solved && challenge.solver.is_some() {
        e.field("🏴‍ Solved", challenge.solver.unwrap(), true);
    }
}

pub async fn create_embed_of_challenge_solved(
    challenge: &Challenge,
    channel_id: &ChannelId,
    http: &Http,
    scoreboard_position: String,
    score: i32,
) -> CommandResult {
    // This should never not be populated
    let solver_name = challenge.solver.as_ref().unwrap();

    channel_id
        .send_message(http, |message| {
            message.embed(|e| {
                e.title(format!(
                    "🏴‍ {} has been solved by {} 🏴‍",
                    challenge.name, solver_name
                ));
                e.description(format!(
                    "📈 New team position: {}, Total score: {}",
                    scoreboard_position, score
                ));
                e.field("📚 Category", &challenge.category, true);
                e.field("💰 Points", &challenge.points, true);
                e
            })
        })
        .await?;

    Ok(())
}

async fn process_solve(
    ctfd_service: &ChallengeProviderService,
    solve: Challenge,
    channel_id: &ChannelId,
    http: &Http,
) -> Result<(), Error> {
    let team_stats = ctfd_service.team_stats().await?;

    // Only try to create an embed if the channel ID isn't 0
    if channel_id.0 != 0 {
        if let Err(why) = create_embed_of_challenge_solved(
            &solve,
            &channel_id,
            http,
            team_stats.place,
            team_stats.score,
        )
        .await
        {
            return Err(format_err!(
                "Error when creating embed for challenge solve: {}",
                why
            ));
        }
    }

    // If it makes it to this point, it will mark it as 'announced_solved' which basically means "processed"
    mark_solved(&solve).await?;

    Ok(())
}

// This needs to be on the tokio runtime so that it can use the serenity framework
#[tokio::main]
pub async fn new_solve_poller_task(http: &Http) {
    let active_ctfs = get_active_ctfs().await.expect("Unable to get active CTFs");

    for ctf in active_ctfs {
        println!("POLLER: Polling CTF: {} for new solves...", ctf.name);
        let solves = check_for_new_solves(&ctf).await;
        let channel_id = ChannelId(ctf.channel_id as u64);
        let ctfd_service = CTFD_CACHE
            .get(&ctf.id)
            .expect("No CTFDService with this CTF ID...");

        match solves {
            Ok(solves) => {
                if solves.is_empty() {
                    println!("POLLER: No new solves found for: {}", ctf.name);
                } else {
                    for solve in solves {
                        match process_solve(&ctfd_service, solve, &channel_id, http).await {
                            Ok(_) => {
                                println!("POLLER: New solve processed.");
                                break;
                            }
                            Err(why) => {
                                eprintln!("Error when processing solve... {}", why);
                            }
                        }
                    }
                }
            }
            Err(why) => {
                eprintln!("POLLER: Error when fetching new solves {}", why);
            }
        }
    }
}

#[tokio::main]
pub async fn scoreboard_and_scores_task() {
    for entry in CTFD_CACHE.iter() {
        let challenge_provider = entry.value();
        match get_and_store_scoreboard(challenge_provider).await {
            Ok(_) => {
                println!("Scoreboard stored successfully...");
            }
            Err(why) => {
                eprintln!(
                    "Error when getting and storing new scoreboard status: {}...",
                    why
                );
            }
        }

        match update_challenges_and_scores(challenge_provider).await {
            Ok(_) => {
                println!("Challenges & their scores updated successfully...");
            }
            Err(why) => {
                eprintln!("Error when updating challenges/scores: {}...", why);
            }
        }
    }
}
