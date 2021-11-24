use std::collections::HashSet;

use chrono::{Local, NaiveDateTime};
use dashmap::DashMap;
use diesel::{insert_into, prelude::*, update};
use diesel::{QueryDsl, RunQueryDsl};
use failure::Error;
use once_cell::sync::Lazy;

use crate::htb::structs::SolveToAnnounce;
use crate::models::HTBSolve;
use crate::models::{HTBRank, HTBUserMapping};
use crate::{
    get_pooled_connection, models::HTBChallenge, schema::htb_challenges::dsl as htb_dsl,
    schema::htb_solves::dsl as htb_solve_dsl, schema::htb_team_rank::dsl as htb_rank_dsl,
    schema::htb_user_id_mapping::dsl as htb_user_mapping_dsl,
};
use crate::{DiscordNameProvider, PooledMysqlConnection};

use super::structs::{GetRecentTeamActivityData, HTBApi, ListActiveChallengesData, RankStats};

pub static CATEGORY_CACHE: Lazy<DashMap<i32, String>> = Lazy::new(DashMap::new);

pub async fn map_htb_response_to_challenge(
    connection: &PooledMysqlConnection,
    challenge: &GetRecentTeamActivityData,
) -> Result<HTBChallenge, Error> {
    let result = htb_dsl::htb_challenges
        .filter(htb_dsl::htb_id.eq(challenge.id))
        .filter(htb_dsl::name.eq(&challenge.name))
        .limit(1)
        .load::<HTBChallenge>(connection)?;

    if !result.is_empty() {
        return Ok(result[0].clone());
    }

    Err(format_err!("Failed to map challenge! {:#?}", challenge))
}

pub fn update_working(
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

pub fn get_challenge_from_id_with_connection(
    id: i32,
    connection: &MysqlConnection,
) -> Result<Vec<HTBChallenge>, Error> {
    let challenges = htb_dsl::htb_challenges
        .filter(htb_dsl::htb_id.eq(id))
        .limit(1)
        .load::<HTBChallenge>(connection)?;
    Ok(challenges)
}

pub async fn get_challenge_from_id(id: i32) -> Result<Vec<HTBChallenge>, Error> {
    let connection = get_pooled_connection().await?;
    get_challenge_from_id_with_connection(id, &connection)
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
    let challenges = get_challenge_from_name(challenge_name, &connection)?;

    if let Some(challenge) = challenges.first() {
        let challenge_id = challenge.htb_id;

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
    let challenges = get_challenge_from_name(challenge_name, &connection)?;
    if let Some(challenge) = challenges.first() {
        return remove_working_from_challenge(username, challenge, &connection);
    }

    Err(format_err!("No challenge with that name found!"))
}

pub fn remove_working_from_challenge(
    username: String,
    challenge: &HTBChallenge,
    connection: &MysqlConnection,
) -> Result<(), Error> {
    let challenge_id = challenge.htb_id;

    if let Some(working) = &challenge.working {
        let mut split: HashSet<String> = working.split(", ").map(str::to_string).collect();

        // Time to check if the user actually exists in here
        if split.contains(&username) {
            split.remove(&username);

            if split.is_empty() {
                update_working(None, challenge_id, connection)?;
            } else {
                let update_value = split.into_iter().collect::<Vec<String>>().join(", ");
                update_working(Some(&update_value), challenge_id, connection)?;
            }
        }
    }

    Ok(())
}

pub async fn process_new_solves(
    api: &HTBApi,
    discord_name_provider: &dyn DiscordNameProvider,
) -> Result<(), Error> {
    let connection = get_pooled_connection().await?;
    let recent_solves = api.get_recent_team_activity().await?;

    for solve in recent_solves {
        if let Ok(challenge) = map_htb_response_to_challenge(&connection, &solve).await {
            if !is_challenge_solved_and_not_announced_for_user(
                solve.user.id,
                challenge.htb_id,
                &solve.solve_type,
                &connection,
            ) {
                let solve_user = &solve.user;

                // Try convert the HTB ID to a Discord user, otherwise just use their HTB username.
                let solver_name = match get_discord_id_for(solve_user.id).await {
                    Ok(discord_id) => discord_name_provider
                        .name_for_id(discord_id)
                        .await
                        .unwrap_or_else(|| solve_user.name.clone()),
                    Err(_) => solve_user.name.clone(),
                };

                println!(
                    "HTB: Adding solve for user {}, challenge: {}",
                    solve_user.name, solve.name
                );

                add_challenge_solved_for_user(
                    solve_user.id,
                    &solver_name,
                    &solve.date,
                    solve.id,
                    &solve.solve_type,
                    &connection,
                )?;
            }
        }
    }

    Ok(())
}

pub async fn get_solves_to_announce() -> Result<Vec<SolveToAnnounce>, Error> {
    let connection = get_pooled_connection().await?;

    let mut solves_to_announce = vec![];

    for solve in get_unannounced_solves(&connection)? {
        let challenges = get_challenge_from_id_with_connection(solve.challenge_id, &connection)?;

        if !challenges.is_empty() {
            let solve_to_announce = SolveToAnnounce {
                solver: solve.username,
                user_id: solve.user_id,
                solve_type: solve.solve_type,
                challenge: challenges[0].clone(),
            };

            solves_to_announce.push(solve_to_announce);
        }
    }

    Ok(solves_to_announce)
}

pub fn get_solves_for_user(
    user_id: i32,
    connection: &PooledMysqlConnection,
) -> Result<Vec<HTBSolve>, Error> {
    let solves = htb_solve_dsl::htb_solves
        .filter(htb_solve_dsl::user_id.eq(user_id))
        .load::<HTBSolve>(connection)?;
    Ok(solves)
}

pub async fn get_solves_for_username(username: &str) -> Result<Vec<HTBSolve>, Error> {
    let connection = get_pooled_connection().await?;
    let solves = htb_solve_dsl::htb_solves
        .filter(htb_solve_dsl::username.eq(username))
        .load::<HTBSolve>(&connection)?;
    Ok(solves)
}

pub async fn get_solving_users_for_challenge(challenge_id: i32) -> Result<Vec<String>, Error> {
    let mut solving_users = vec![];

    let connection = get_pooled_connection().await?;

    let solves = get_solves_for_challenge(challenge_id, &connection)?;

    if solves.is_empty() {
        return Err(format_err!("No solves for this challenge found!"));
    }

    for solve in solves {
        // This can be doubled because user and root on machines are stored seperate
        if !solving_users.contains(&solve.username) {
            solving_users.push(solve.username);
        }
    }

    Ok(solving_users)
}

pub fn get_solves_for_challenge(
    challenge_id: i32,
    connection: &PooledMysqlConnection,
) -> Result<Vec<HTBSolve>, Error> {
    let solves = htb_solve_dsl::htb_solves
        .filter(htb_solve_dsl::challenge_id.eq(challenge_id))
        .load::<HTBSolve>(connection)?;
    Ok(solves)
}

pub fn get_unannounced_solves(connection: &PooledMysqlConnection) -> Result<Vec<HTBSolve>, Error> {
    let solves = htb_solve_dsl::htb_solves
        .filter(htb_solve_dsl::announced.eq(false))
        .load::<HTBSolve>(connection)?;
    Ok(solves)
}

pub fn add_challenge_solved_for_user(
    user_id: i32,
    username: &str,
    solve_date: &str,
    challenge_id: i32,
    solve_type: &str,
    connection: &MysqlConnection,
) -> Result<(), Error> {
    let challenges = get_challenge_from_id_with_connection(challenge_id, connection)?;

    if !challenges.is_empty() {
        let challenge = &challenges[0];
        let solved_time = NaiveDateTime::parse_from_str(solve_date, "%Y-%m-%dT%H:%M:%S.%Z")?;

        insert_into(htb_solve_dsl::htb_solves)
            .values((
                htb_solve_dsl::user_id.eq(user_id),
                htb_solve_dsl::username.eq(username),
                htb_solve_dsl::challenge_id.eq(challenge.htb_id),
                htb_solve_dsl::announced.eq(false),
                htb_solve_dsl::solve_type.eq(solve_type),
                htb_solve_dsl::solved_time.eq(solved_time),
            ))
            .execute(connection)?;

        // Remove user as working once they have solved
        return remove_working_from_challenge(username.to_string(), challenge, connection);
    }

    Err(format_err!("No challenge by that ID was found!"))
}

pub async fn add_challenge_announced_for_user(
    solve: &SolveToAnnounce,
    challenge_id: i32,
) -> Result<(), Error> {
    let connection = get_pooled_connection().await?;
    let challenges = get_challenge_from_id_with_connection(challenge_id, &connection)?;

    if !challenges.is_empty() {
        let challenge = &challenges[0];

        update(htb_solve_dsl::htb_solves)
            .filter(htb_solve_dsl::user_id.eq(solve.user_id))
            .filter(htb_solve_dsl::challenge_id.eq(challenge.htb_id))
            .set(htb_solve_dsl::announced.eq(true))
            .execute(&connection)?;

        return Ok(());
    }

    Err(format_err!("No challenge by that ID was found!"))
}

pub fn is_challenge_solved_and_not_announced_for_user(
    user_id: i32,
    challenge_id: i32,
    solve_type: &str,
    connection: &MysqlConnection,
) -> bool {
    if let Ok(solves) = htb_solve_dsl::htb_solves
        .filter(htb_solve_dsl::user_id.eq(user_id))
        .filter(htb_solve_dsl::challenge_id.eq(challenge_id))
        .filter(htb_solve_dsl::solve_type.eq(solve_type))
        .filter(htb_solve_dsl::announced.eq(true))
        .limit(1)
        .load::<HTBSolve>(connection)
    {
        return !solves.is_empty();
    }

    false
}

pub async fn update_htb_challenges_and_scores(htb_api: &HTBApi) -> Result<(), Error> {
    let connection = get_pooled_connection().await?;

    let challenges = htb_api.list_active_challenges().await?;
    for challenge in challenges.challenges {
        // We don't need to update the score for challenges, they are static
        ensure_challenge_exists_otherwise_add(&challenge, &connection).await?;
    }

    let machines = htb_api.list_active_machines().await?.info;
    for machine in machines {
        let machine_points = format!("{}", machine.points);
        let mapped_to_challenge_data = ListActiveChallengesData {
            id: machine.id,
            name: machine.name,
            difficulty: machine.difficulty,
            points: machine_points,
            release_date: machine.release,
            challenge_category_id: 100,
            machine_avatar: Some(machine.avatar),
        };

        ensure_challenge_exists_otherwise_add(&mapped_to_challenge_data, &connection).await?;
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
        println!("HTB: Found a challenge that we haven't got, adding now...");
        insert_into(htb_dsl::htb_challenges)
            .values((
                htb_dsl::htb_id.eq(challenge.id),
                htb_dsl::name.eq(&challenge.name),
                htb_dsl::difficulty.eq(&challenge.difficulty),
                htb_dsl::points.eq(&challenge.points),
                htb_dsl::release_date.eq(&challenge.release_date),
                htb_dsl::challenge_category.eq(&challenge.challenge_category_id),
                htb_dsl::machine_avatar.eq(&challenge.machine_avatar),
            ))
            .execute(connection)?;

        return Ok(true);
    }

    Ok(false)
}

pub async fn insert_rank_into_db(rank_stats: &RankStats) -> Result<(), Error> {
    let connection = get_pooled_connection().await?;

    insert_into(htb_rank_dsl::htb_team_rank)
        .values((
            htb_rank_dsl::rank.eq(&rank_stats.data.rank),
            htb_rank_dsl::points.eq(&rank_stats.data.points),
        ))
        .execute(&connection)?;

    Ok(())
}

pub async fn get_latest_rank_from_db() -> Result<HTBRank, Error> {
    let connection = get_pooled_connection().await?;

    let solves = htb_rank_dsl::htb_team_rank
        .order(htb_rank_dsl::entry_id.desc())
        .limit(1)
        .load::<HTBRank>(&connection)?;

    match solves.first() {
        Some(first) => Ok(first.clone()),
        None => Ok(HTBRank {
            entry_id: 0,
            rank: 0,
            points: 0,
            timestamp: Local::now().naive_local(),
        }),
    }
}

pub async fn get_discord_id_for(htb_id: i32) -> Result<i64, Error> {
    let connection = get_pooled_connection().await?;

    let user = htb_user_mapping_dsl::htb_user_id_mapping
        .filter(htb_user_mapping_dsl::htb_id.eq(htb_id))
        .load::<HTBUserMapping>(&connection)?;

    if let Some(first) = user.first() {
        return Ok(first.discord_id);
    }

    Err(format_err!(
        "No discord id found for HTB user with ID: {}!",
        htb_id
    ))
}

pub async fn get_htb_name_for(htb_id: i32, htb_api: &HTBApi) -> Result<String, Error> {
    Ok(htb_api.get_user_overview(htb_id).await?.profile.name)
}

pub async fn set_discord_id_for(htb_id: i32, discord_id: i64) -> Result<(), Error> {
    let connection = get_pooled_connection().await?;

    // First need to check to see if there exists a mapping with the given HTB ID.
    let results = htb_user_mapping_dsl::htb_user_id_mapping
        .filter(htb_user_mapping_dsl::htb_id.eq(htb_id))
        .load::<HTBUserMapping>(&connection)?;

    // If results is empty, meaning that there was no user mapping found with the given id, then
    // We need to insert it into the database.
    if results.is_empty() {
        insert_into(htb_user_mapping_dsl::htb_user_id_mapping)
            .values((
                htb_user_mapping_dsl::htb_id.eq(htb_id),
                htb_user_mapping_dsl::discord_id.eq(discord_id),
            ))
            .execute(&connection)?;

        Ok(())
    } else {
        // If the user mapping entry exists, then we just need to update it.
        update(htb_user_mapping_dsl::htb_user_id_mapping)
            .filter(htb_user_mapping_dsl::htb_id.eq(htb_id))
            .set(htb_user_mapping_dsl::discord_id.eq(discord_id))
            .execute(&connection)?;

        Ok(())
    }
}

#[tokio::main]
pub async fn load_categories_to_cache(htb_api: &HTBApi) -> Result<(), Error> {
    let challenge_categories_response = htb_api.get_challenge_categories().await?;

    for category in challenge_categories_response.info {
        CATEGORY_CACHE.insert(category.id, category.name);
    }

    // Machines don't have a category, so we add one with an ID that wont collide
    CATEGORY_CACHE.insert(100, "Machine".to_owned());

    Ok(())
}
