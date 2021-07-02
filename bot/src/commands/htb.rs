use ctfdb::htb::db::{add_working, remove_working, search_for_challenge_by_name};
use serenity::client::Context;
use serenity::framework::standard::{macros::*, Args, CommandResult};

use serenity::model::channel::Message;

use crate::populate_embed_from_htb_challenge;

#[group]
#[commands(working, giveup, search)]
#[prefixes("htb", "h")]
pub struct Hacker;

#[command]
#[allowed_roles("hacker")]
#[aliases("w")]
#[example("\"Challenge name\"")]
#[description = "Marks you as working on the provided challenge"]
async fn working(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.len() == 1 {
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
    } else {
          msg.reply(&ctx.http, "Usage: ``!htb working \"Challenge name\"``")
            .await?;
    }

    Ok(())
}

// For lack of a better name...
#[command]
#[allowed_roles("hacker")]
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
        msg.reply(&ctx.http, "Usage: ``!htb giveup \"Challenge name\"``")
            .await?;
    }

    Ok(())
}

#[command]
#[allowed_roles("hacker")]
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
        msg.reply(&ctx.http, "Usage: ``!htb search \"Challenge name\"``")
            .await?;
    }

    Ok(())
}
