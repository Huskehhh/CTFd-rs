extern crate ctfdb;
extern crate dotenv;

use std::{
    collections::HashSet,
    env,
    thread::{self, sleep},
    time::Duration,
};

use dotenv::dotenv;
use serenity::framework::standard::{
    help_commands, macros::*, Args, CommandGroup, CommandResult, DispatchError, HelpOptions,
};
use serenity::framework::StandardFramework;
use serenity::model::channel::Message;
use serenity::{async_trait, http::Http, model::id::UserId, Client};
use serenity::{
    client::{Context, EventHandler},
    model::prelude::Activity,
};
use serenity::{model::gateway::Ready, model::Permissions};

use ctf_bot::{new_solve_poller_task, populate_embed_from_challenge, scoreboard_and_scores_task};
use ctfdb::{
    add_active_ctf, add_working, get_active_ctfs, get_challenges_for_channel,
    get_challenges_for_ctfname, get_latest_scoreboard_status, initial_load_tasks,
    remove_active_ctf, remove_working, search_for_challenge_by_name,
};

#[help]
#[individual_command_tip = "If you want more information about a specific command, just pass the command as argument.\n Example: !help ctf list\n"]
#[command_not_found_text = "Could not find: `{}`."]
#[max_levenshtein_distance(3)]
#[strikethrough_commands_tip_in_guild = ""]
#[strikethrough_commands_tip_in_dm = ""]
#[indention_prefix = "+"]
#[lacking_permissions = "Hide"]
#[lacking_role = "Nothing"]
async fn help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[group]
#[commands(active, working, giveup, start, end, list, search, stats)]
struct CTFer;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        ctx.set_activity(Activity::playing("View at https://ctf.husk.pro/"))
            .await;

        match ready.user.invite_url(&ctx.http, Permissions::empty()).await {
            Ok(url) => {
                println!("You can invite me using this url! {}", &url);
            }
            Err(why) => {
                eprintln!("Error getting invite url: {:?}", why);
            }
        };
    }
}

#[command]
#[allowed_roles("Organiser")]
#[aliases("start")]
#[example("\"CTF name\" <ctf url> <api key>")]
#[example("\"CTF name\" <ctf url> <api key> <channel id to post updates to>")]
#[description = "Starts a CTF, and will begin polling for challenge status changes"]
async fn start(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.len() >= 3 {
        let name = args.single_quoted::<String>()?;
        let base_url = args.single::<String>()?;
        let api_key = args.single::<String>()?;
        let channel_id = args.single::<i64>().unwrap_or(0);

        let api_url = match base_url.ends_with('/') {
            true => {
                format!("{}api/v1", base_url)
            }
            false => {
                format!("{}/api/v1", base_url)
            }
        };

        match add_active_ctf(&name, &base_url, &api_url, &api_key, channel_id).await {
            Ok(_) => {
                let started_ctf_msg = format!("Started CTF '{}'", name);
                msg.reply(&ctx.http, started_ctf_msg).await?;
            }
            Err(why) => {
                msg.channel_id
                    .send_message(&ctx.http, |m| {
                        m.content(&format!(
                            "Error occurred when adding new active CTF... try again?: {}",
                            why
                        ));
                        m
                    })
                    .await?;
                eprintln!("Error occurred when adding new active ctf: {}", why);
            }
        }
    } else {
        msg.reply(
            &ctx.http,
            "Usage: ``!start \"CTF Name\" <ctf url> <ctf api key> <id of channel to post updates in>``",
        )
            .await?;
    }

    Ok(())
}

#[command]
#[allowed_roles("Organiser")]
#[example("\"CTF name\"")]
#[description = "Ends a CTF with the provided name"]
async fn end(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let name = args.single_quoted::<String>()?;

    match remove_active_ctf(&name).await {
        Ok(_) => {
            msg.reply(&ctx.http, &format!("CTF ended '{}'", name))
                .await?;
            // TODO show results
        }
        Err(why) => {
            eprintln!("Error occurred when ending active ctf: {}", why);
        }
    }

    Ok(())
}

#[command]
#[allowed_roles("CTFer")]
#[description = "Shows all active CTFs"]
async fn active(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    match get_active_ctfs().await {
        Ok(active_ctfs) => {
            for ctf in active_ctfs {
                msg.channel_id
                    .send_message(&ctx.http, |message| message.embed(|e| e.title(ctf.name)))
                    .await?;
            }
        }
        Err(why) => {
            let err_str = format!("Error occurred when listing active CTFs! {}", why);
            msg.reply(&ctx.http, &err_str).await?;
            eprintln!("{}", err_str);
        }
    }

    Ok(())
}

#[command]
#[allowed_roles("CTFer")]
#[aliases("w")]
#[example("\"Challenge name\"")]
#[description = "Marks you as working on the provided challenge"]
async fn working(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let challenge_name = args.single_quoted::<String>()?;

    let username = msg
        .author_nick(&ctx.http)
        .await
        .unwrap_or_else(|| msg.author.name.clone());

    match add_working(username, &challenge_name).await {
        Ok(_) => {
            msg.reply(
                &ctx.http,
                &format!("Marked you as working on '{}'", &challenge_name),
            )
            .await?;
        }
        Err(why) => {
            msg.reply(
                &ctx.http,
                format!(
                    "Error when adding to working for '{}'... {}",
                    &challenge_name, why
                ),
            )
            .await?;
            eprintln!(
                "Error on adding to working for '{}' ... '{}'",
                &msg.author.name, why
            );
        }
    }

    Ok(())
}

// For lack of a better name...
#[command]
#[allowed_roles("CTFer")]
#[aliases("g")]
#[example("\"Challenge name\"")]
#[description = "Removes you from working on the given challenge"]
async fn giveup(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.len() == 1 {
        let challenge_name = args.single_quoted::<String>()?;
        let username = msg
            .author_nick(&ctx.http)
            .await
            .unwrap_or_else(|| msg.author.name.clone());

        remove_working(username, &challenge_name).await?;

        msg.reply(
            &ctx.http,
            &format!("Removed you from working on '{}'", &challenge_name),
        )
        .await?;
    } else {
        msg.reply(&ctx.http, "Usage: ``!giveup \"Challenge name\"``")
            .await?;
    }

    Ok(())
}

#[command]
#[allowed_roles("CTFer")]
#[aliases("l")]
#[example("\"CTF Name\"")]
#[description = "Lists all challenges for given CTF, or defaults to checking for the active CTF in the current channel"]
async fn list(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut challenges = vec![];
    if args.len() == 1 {
        let ctf_name = args.single_quoted::<String>()?;
        challenges = get_challenges_for_ctfname(ctf_name).await?;
    } else if args.is_empty() {
        match get_challenges_for_channel(msg.channel_id.0 as i64).await {
            Ok(challs) => {
                challenges = challs;
            }
            Err(why) => {
                eprintln!("Error when trying to get challenges for channel: {}", why);
            }
        }
    }

    if !challenges.is_empty() {
        for challenge in challenges {
            msg.author
                .dm(&ctx.http, |m| {
                    m.embed(|e| {
                        populate_embed_from_challenge(challenge, e);
                        e
                    })
                })
                .await?;
        }
    } else {
        msg.reply(
            &ctx.http,
            "Usage: ``!list \"CTF name\"`` or use ``!list`` in the correct channel",
        )
        .await?;
    }

    Ok(())
}

#[command]
#[allowed_roles("CTFer")]
#[example("\"Challenge name\"")]
#[description = "Searches for the status of the given challenge"]
async fn search(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.len() == 1 {
        let challenge_name = args.single_quoted::<String>()?;

        let challenges = search_for_challenge_by_name(&challenge_name).await?;

        if !challenges.is_empty() {
            for challenge in challenges {
                msg.channel_id
                    .send_message(&ctx.http, |m| {
                        m.embed(|e| {
                            populate_embed_from_challenge(challenge, e);
                            e
                        })
                    })
                    .await?;
            }
        } else {
            msg.reply(&ctx.http, "No challenge found by that name!")
                .await?;
        }
    } else {
        msg.reply(&ctx.http, "Usage: ``!search \"Challenge name\"")
            .await?;
    }

    Ok(())
}

#[command]
#[allowed_roles("CTFer")]
#[description = "Displays the stats for all active ctfs"]
async fn stats(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let active_ctfs = get_active_ctfs().await?;

    for ctf in active_ctfs {
        let stats = get_latest_scoreboard_status(ctf.id).await?;

        msg.channel_id
            .send_message(&ctx.http, |message| {
                message.embed(|e| {
                    e.title(ctf.name);
                    e.description(format!(
                        "📈 Team position: {}, Total score: {}",
                        stats.position, stats.points
                    ));
                    e
                })
            })
            .await?;
    }

    Ok(())
}

#[hook]
async fn normal_message(_ctx: &Context, msg: &Message) {
    println!("{}: {}", msg.author.name, msg.content);
}

#[hook]
async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError) {
    if let DispatchError::Ratelimited(info) = error {
        if info.is_first_try {
            let _ = msg
                .channel_id
                .say(
                    &ctx.http,
                    &format!("Try this again in {} seconds.", info.as_secs()),
                )
                .await;
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token =
        env::var("DISCORD_TOKEN").expect("Expected a token in your environment (DISCORD_TOKEN)");
    let owner_id_str = env::var("OWNER_ID").expect("Expected an OWNER_ID in your environment!");

    // Load all the active ctfs into memory
    if let Err(why) = initial_load_tasks().await {
        eprintln!("Error when running initial load tasks {}", why);
    }

    let owner_id = owner_id_str.parse::<u64>().expect("Unable to parse OWNER_ID into u64... Did you put it in correctly?");

    let mut owners = HashSet::new();
    owners.insert(UserId(owner_id));

    let framework = StandardFramework::new()
        .configure(|c| {
            c.prefixes(vec!["!", ".", "~"])
                .owners(owners)
                .with_whitespace(true)
        })
        .normal_message(normal_message)
        .on_dispatch_error(dispatch_error)
        .help(&HELP)
        .group(&CTFER_GROUP);

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    thread::spawn(move || {
        let http = Http::new_with_token(&token);
        loop {
            new_solve_poller_task(&http);
            sleep(Duration::from_secs(15));
        }
    });

    thread::spawn(move || loop {
        scoreboard_and_scores_task();
        sleep(Duration::from_secs(60));
    });

    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}
