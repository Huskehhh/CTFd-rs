#[macro_use]
extern crate failure;

use failure::Error;
use futures::executor::block_on;
use serenity::{
    builder::CreateEmbed, framework::standard::CommandResult, http::Http, model::id::ChannelId,
};

use ctfdb::{
    ctfs::db::{
        check_for_new_solves, get_active_ctfs, get_and_store_scoreboard, mark_solved,
        update_challenges_and_scores, CTF_CACHE,
    },
    htb::{
        db::{
            add_challenge_announced_for_user, get_latest_rank_from_db, get_solves_to_announce,
            get_solving_users_for_challenge, insert_rank_into_db, process_new_solves,
            update_htb_challenges_and_scores, CATEGORY_CACHE,
        },
        structs::{HTBApi, RankStatsData, SolveToAnnounce},
    },
    models::{Challenge, HTBChallenge},
    ChallengeProvider, DiscordNameProvider,
};

pub mod commands;
pub mod discord_name_provider;

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

pub fn populate_embed_from_htb_challenge(challenge: HTBChallenge, e: &mut CreateEmbed) {
    let challenge_category_name = get_challenge_category_from_id(challenge.challenge_category);

    e.title(format!("❓ {} ❓", challenge.name));
    e.field("📚 Category", &challenge_category_name, true);
    e.field("💰 Points", challenge.points, true);

    if challenge.working.is_some() {
        e.field("🧰 Working", challenge.working.unwrap(), true);
    }

    if let Ok(solving_users) = block_on(get_solving_users_for_challenge(challenge.htb_id)) {
        let solving_string = solving_users.join(", ");
        e.field("🏴‍ Solved", solving_string, true);
    }
}

pub fn get_challenge_category_from_id(challenge_category_id: i32) -> String {
    match CATEGORY_CACHE.get(&challenge_category_id) {
        Some(cached) => cached.value().clone(),
        None => "Unknown".to_string(),
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
                    "🏴‍ {} has been solved by {}‍",
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

// Cheeky little function to capitalise the first char of a string.
fn capitalise_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}

pub async fn create_embed_of_htb_challenge_solved(
    solve: &SolveToAnnounce,
    channel_id: &ChannelId,
    http: &Http,
) -> CommandResult {
    let challenge = &solve.challenge;
    // This should never not be populated
    let challenge_category_name = get_challenge_category_from_id(challenge.challenge_category);

    let solve_type = capitalise_first(&solve.solve_type);

    // Build up the content based around the solve type
    let content;
    if solve_type.eq("Challenge") {
        content = format!(
            "🏴 {} has been solved by {}",
            &challenge.name, &solve.solver
        );
    } else {
        content = format!(
            "🏴 {} has been owned by {} on {}",
            solve_type, &solve.solver, &challenge.name
        );
    }

    channel_id
        .send_message(http, |message| {
            message.embed(|e| {
                e.title(content);
                e.field("📚 Category", &challenge_category_name, true);
                e.field("💰 Points", &challenge.points, true);

                if let Some(avatar) = &solve.challenge.machine_avatar {
                    e.thumbnail(format!("https://www.hackthebox.eu/{}", avatar));
                }

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
            channel_id,
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

async fn process_htb_solve(
    solve: SolveToAnnounce,
    channel_id: &ChannelId,
    http: &Http,
) -> Result<(), Error> {
    // Only try to create an embed if the channel ID isn't 0
    if channel_id.0 != 0 {
        if let Err(why) = create_embed_of_htb_challenge_solved(&solve, channel_id, http).await {
            return Err(format_err!(
                "Error when creating embed for challenge solve: {}",
                why
            ));
        }
    }

    // If it makes it to this point, it will mark it as 'announced_solved' which basically means "processed"
    add_challenge_announced_for_user(&solve, solve.challenge.htb_id).await?;

    Ok(())
}

pub async fn process_rank_status(
    htb_api: &HTBApi,
    channel_id: &ChannelId,
    http: &Http,
) -> Result<(), Error> {
    let latest_rank = htb_api.get_team_rank().await?;

    let current_rank = get_latest_rank_from_db().await?;

    if latest_rank.data.rank != current_rank.rank || latest_rank.data.points != current_rank.points
    {
        insert_rank_into_db(&latest_rank).await?;

        if let Err(why) =
            update_htb_channel_topic_with_stats(&latest_rank.data, channel_id, http).await
        {
            eprintln!("Error when creating embed of team stats... {}", why);
        }
    }

    Ok(())
}

pub async fn update_htb_channel_topic_with_stats(
    stats: &RankStatsData,
    channel_id: &ChannelId,
    http: &Http,
) -> Result<(), Error> {
    let new_channel_topic = format!("Team rank {}, Points: {}", stats.rank, stats.points);
    match channel_id.edit(&http, |c| c.topic(new_channel_topic)).await {
        Ok(_) => Ok(()),
        Err(why) => Err(format_err!("Error when updating channel topic: {}", why)),
    }
}

// This needs to be on the tokio runtime so that it can use the serenity framework
#[tokio::main]
pub async fn new_solve_poller_task(http: &Http) {
    let active_ctfs = get_active_ctfs().await.expect("Unable to get active CTFs");

    for ctf in active_ctfs {
        println!("POLLER: Polling CTF: {} for new solves...", ctf.name);
        let solves = check_for_new_solves(&ctf).await;
        let channel_id = ChannelId(ctf.channel_id as u64);
        if let Some(ctfd_service) = CTF_CACHE.get(&ctf.id) {
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
}

#[tokio::main]
pub async fn scoreboard_and_scores_task() {
    for entry in CTF_CACHE.iter() {
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

#[tokio::main]
pub async fn htb_poller_task(
    htb_api: &mut HTBApi,
    http: &Http,
    channel_id: &ChannelId,
    discord_name_provider: &dyn DiscordNameProvider,
) -> Result<(), Error> {
    htb_api.handle_token_renewal().await?;
    update_htb_challenges_and_scores(htb_api).await?;
    process_new_solves(htb_api, discord_name_provider).await?;
    process_rank_status(htb_api, channel_id, http).await?;

    let solves = get_solves_to_announce().await;
    match solves {
        Ok(solves) => {
            if solves.is_empty() {
                println!("HTB POLLER: No new solves found for HTB.");
            } else {
                for solve in solves {
                    match process_htb_solve(solve, channel_id, http).await {
                        Ok(_) => {
                            println!("HTB POLLER: New solve processed.");
                        }
                        Err(why) => {
                            eprintln!("Error when processing HTB solve... {}", why);
                        }
                    }
                }
            }
        }
        Err(why) => {
            eprintln!("HTB POLLER: Error when fetching new solves {}", why);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capitalise_first() {
        assert_eq!("Yes", capitalise_first("yes"));
        assert_eq!("Yes", capitalise_first("Yes"));
    }
}
