use std::collections::HashSet;

use ctfdb::htb::db::{add_working, remove_working, search_for_challenge_by_name};
use serenity::client::Context;
use serenity::framework::standard::{
    help_commands, macros::*, Args, CommandGroup, CommandResult, HelpOptions,
};

use serenity::model::channel::Message;
use serenity::model::id::UserId;

use crate::populate_embed_from_htb_challenge;

#[group]
#[commands(working, giveup, search)]
#[prefixes("htb", "h")]
pub struct Hacker;

#[help]
#[individual_command_tip = "If you want more information about a specific command, just pass the command as argument.\n Example: !help htb list\n"]
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

#[command]
#[allowed_roles("Hacker")]
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
#[allowed_roles("Hacker")]
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
#[allowed_roles("Hacker")]
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
                            populate_embed_from_htb_challenge(challenge, e);
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
        msg.reply(&ctx.http, "Usage: ``!search \"Challenge name\"``")
            .await?;
    }

    Ok(())
}
