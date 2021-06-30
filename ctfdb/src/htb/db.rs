use chrono::NaiveDateTime;
use dashmap::DashMap;
use diesel::{insert_into, prelude::*, update};
use diesel::{QueryDsl, RunQueryDsl};
use failure::Error;
use once_cell::sync::Lazy;

use crate::PooledMysqlConnection;
use crate::{get_pooled_connection, models::HTBChallenge, schema::htb_challenges::dsl as htb_dsl};

use super::structs::{GetRecentTeamActivityData, HTBApi, ListActiveChallengesData};

pub static CATEGORY_CACHE: Lazy<DashMap<i32, String>> = Lazy::new(DashMap::new);

pub async fn map_htb_response_to_challenge(
    connection: &PooledMysqlConnection,
    challenge: &GetRecentTeamActivityData,
) -> Result<HTBChallenge, Error> {
    let result = htb_dsl::htb_challenges
        .filter(htb_dsl::htb_id.eq(challenge.id))
        .limit(1)
        .load::<HTBChallenge>(connection)?;

    if !result.is_empty() {
        return Ok(result[0].clone());
    }

    Err(format_err!("Failed to map challenge! {:#?}", challenge))
}

fn update_working(
    update_value: Option<&str>,
    challenge_id: i32,
    connection: &MysqlConnection,
) -> Result<(), Error> {
    update(htb_dsl::htb_challenges)
        .filter(htb_dsl::htb_id.eq(challenge_id))
        .set(htb_dsl::working.eq(update_value))
        .execute(connection)?;
    Ok(())
}

pub fn get_challenge_from_name(
    name: &str,
    connection: &MysqlConnection,
) -> Result<Vec<HTBChallenge>, Error> {
    let challenges = htb_dsl::htb_challenges
        .filter(htb_dsl::name.eq(name))
        .limit(1)
        .load::<HTBChallenge>(connection)?;
    Ok(challenges)
}

pub async fn search_for_challenge_by_name(name: &str) -> Result<Vec<HTBChallenge>, Error> {
    let connection = get_pooled_connection().await?;
    let search = format!("%{}%", name);
    let challenges = htb_dsl::htb_challenges
        .filter(htb_dsl::name.like(search))
        .load::<HTBChallenge>(&connection)?;
    Ok(challenges)
}

pub async fn add_working(username: String, challenge_name: &str) -> Result<(), Error> {
    let connection = get_pooled_connection()
        .await
        .expect("Error when getting pooled connection");

    // First load the challenge by that name
    let challenges = get_challenge_from_name(&challenge_name, &connection)?;

    if let Some(challenge) = challenges.first() {
        let challenge_id = challenge.id;

        match &challenge.working {
            Some(working) => {
                let mut split: Vec<String> = working.split(", ").map(str::to_string).collect();
                if !split.contains(&username) {
                    split.push(username);
                    let update_value = split.join(", ");
                    update_working(Some(&update_value), challenge_id, &connection)?;
                }
            }
            None => {
                let working = vec![username];
                let update_value = working.join(", ");
                update_working(Some(&update_value), challenge_id, &connection)?;
            }
        }
    } else {
        return Err(format_err!("No challenge exists under that name!"));
    }

    Ok(())
}

pub async fn remove_working(username: String, challenge_name: &str) -> Result<(), Error> {
    let connection = get_pooled_connection().await?;

    // First load the challenge by that name
    let challenges = get_challenge_from_name(&challenge_name, &connection)?;

    if let Some(challenge) = challenges.first() {
        let challenge_id = challenge.id;

        if let Some(working) = &challenge.working {
            let mut split: Vec<String> = working.split(", ").map(str::to_string).collect();

            // Time to check if the user actually exists in here
            if split.contains(&username) {
                // Remove by index
                let mut index = 0;
                for entry in &split {
                    if entry.eq(&username) {
                        break;
                    }
                    index += 1;
                }
                split.remove(index);

                if split.is_empty() {
                    update_working(None, challenge_id, &connection)?;
                } else {
                    let update_value = split.join(", ");
                    update_working(Some(&update_value), challenge_id, &connection)?;
                }
            }
        }
    }

    Ok(())
}

pub async fn mark_solved(challenge: &HTBChallenge) -> Result<(), Error> {
    let connection = get_pooled_connection().await?;

    let challenge_id = challenge.id;

    update(htb_dsl::htb_challenges)
        .filter(htb_dsl::id.eq(challenge_id))
        .set((
            htb_dsl::solver.eq(&challenge.solver),
            htb_dsl::solved.eq(true),
            htb_dsl::solved_time.eq(&challenge.solved_time),
            htb_dsl::announced_solve.eq(true),
        ))
        .execute(&connection)?;

    Ok(())
}

pub async fn get_new_solves(api: &HTBApi) -> Result<Vec<HTBChallenge>, Error> {
    let connection = get_pooled_connection().await?;
    let mut new_solves = vec![];
    let recent_activity = api.get_recent_team_activity().await?;

    for solve in recent_activity {
        let mut challenge = map_htb_response_to_challenge(&connection, &solve).await?;

        if !challenge.announced_solve {
            let solved_time = NaiveDateTime::parse_from_str(&solve.date, "%Y-%m-%dT%H:%M:%S%z")?;
            let solver_name = solve.user.name;

            challenge.solved_time = Some(solved_time);
            challenge.solver = Some(solver_name);

            new_solves.push(challenge);
        }
    }

    Ok(new_solves)
}

pub async fn update_challenges_and_scores(htb_api: &HTBApi) -> Result<(), Error> {
    let connection = get_pooled_connection().await?;

    let challenges = htb_api.list_active_challenges().await?;
    for challenge in challenges.challenges {
        // We don't need to update the score for challenges, they are static
        ensure_challenge_exists_otherwise_add(&challenge, &connection).await?;
    }

    Ok(())
}

pub async fn ensure_challenge_exists_otherwise_add(
    challenge: &ListActiveChallengesData,
    connection: &MysqlConnection,
) -> Result<bool, Error> {
    let challenges = htb_dsl::htb_challenges
        .filter(htb_dsl::htb_id.eq(&challenge.id))
        .filter(htb_dsl::name.eq(&challenge.name))
        .limit(1)
        .load::<HTBChallenge>(connection)?;

    if challenges.is_empty() {
        insert_into(htb_dsl::htb_challenges)
            .values((
                htb_dsl::htb_id.eq(challenge.id),
                htb_dsl::name.eq(&challenge.name),
                htb_dsl::difficulty.eq(&challenge.difficulty),
                htb_dsl::points.eq(&challenge.points),
                htb_dsl::release_date.eq(&challenge.release_date),
                htb_dsl::challenge_category.eq(&challenge.challenge_category_id),
                htb_dsl::solved.eq(false),
                htb_dsl::announced_solve.eq(false),
            ))
            .execute(connection)?;

        return Ok(true);
    }

    Ok(false)
}

async fn load_categories_to_cache(htb_api: &HTBApi) -> Result<(), Error> {
    let challenge_categories_response = htb_api.get_challenge_categories().await?;

    for category in challenge_categories_response.info {
        CATEGORY_CACHE.insert(category.id, category.name);
    }

    Ok(())
}
