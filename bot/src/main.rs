extern crate ctfdb;
extern crate dotenv;

use std::{
    collections::HashSet,
    env,
    thread::{self, sleep},
    time::Duration,
};

use ctf_bot::{
    commands::ctf::*, commands::htb::*, new_solve_poller_task, scoreboard_and_scores_task,
};
use ctfdb::ctfs::db::initial_load_tasks;
use dotenv::dotenv;
use serenity::{client::Context, framework::standard::{Args, CommandGroup, CommandResult, HelpOptions, help_commands}};
use serenity::framework::standard::{macros::*, DispatchError};
use serenity::framework::StandardFramework;
use serenity::model::channel::Message;
use serenity::{http::Http, model::id::UserId, Client};

use serenity::async_trait;
use serenity::{client::EventHandler, model::prelude::Activity};
use serenity::{model::gateway::Ready, model::Permissions};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        ctx.set_activity(Activity::playing("https://ctf.husk.pro/"))
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

    let owner_id = owner_id_str
        .parse::<u64>()
        .expect("Unable to parse OWNER_ID into u64... Did you put it in correctly?");

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
        .group(&CTFER_GROUP)
        .group(&HACKER_GROUP);

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
