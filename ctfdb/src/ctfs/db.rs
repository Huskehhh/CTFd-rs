use chrono::NaiveDateTime;
use dashmap::DashMap;
use diesel::prelude::*;
use diesel::{insert_into, update, MysqlConnection, QueryDsl, RunQueryDsl};
use failure::Error;
use once_cell::sync::Lazy;

use crate::models::{Challenge, Ctf, Scoreboard};
use crate::schema::challenges::dsl as chall_dsl;
use crate::schema::ctfs::dsl as ctf_dsl;
use crate::schema::scoreboard::dsl as scoreboard_dsl;
use crate::{get_pooled_connection, ChallengeProvider, PooledMysqlConnection};

use super::ctfd::api::*;
use super::structs::*;

pub type ChallengeProviderService = Box<dyn ChallengeProvider + Send + Sync>;

pub static CTF_CACHE: Lazy<DashMap<i32, ChallengeProviderService>> = Lazy::new(DashMap::new);

pub async fn get_active_ctfs() -> Result<Vec<Ctf>, Error> {
    let connection = get_pooled_connection().await?;

    Ok(ctf_dsl::ctfs
        .filter(ctf_dsl::active.eq(true))
        .load::<Ctf>(&connection)?)
}

pub async fn add_active_ctf(
    name: &str,
    base_url: &str,
    api_url: &str,
    api_key: &str,
    channel_id: i64,
) -> Result<(), Error> {
    let connection = get_pooled_connection().await?;

    insert_into(ctf_dsl::ctfs)
        .values((
            ctf_dsl::active.eq(true),
            ctf_dsl::name.eq(name),
            ctf_dsl::base_url.eq(base_url),
            ctf_dsl::api_url.eq(&api_url),
            ctf_dsl::api_key.eq(api_key),
            ctf_dsl::channel_id.eq(channel_id),
        ))
        .execute(&connection)?;

    let service_config = ChallengeProviderServiceConfig {
        name: name.to_string(),
        base_url: base_url.to_string(),
        api_url: api_url.to_string(),
        api_key: api_key.to_string(),
        service_type: ChallengeProviderServiceTypes::Ctfd, // Default as CTFD for now...
    };

    // Create & cache challenge provider service
    let challenge_provider_service = new_ctfdservice(service_config).await;

    // Create & cache all challenges
    initial_create_all_challenges_in_db(&challenge_provider_service).await?;

    CTF_CACHE.insert(
        challenge_provider_service.get_id(),
        challenge_provider_service,
    );

    Ok(())
}

pub async fn initial_create_all_challenges_in_db(
    challenge_provider: &ChallengeProviderService,
) -> Result<(), Error> {
    let connection = get_pooled_connection().await?;

    let challenges = challenge_provider.get_challenges().await?;
    for challenge in challenges {
        insert_into(chall_dsl::challenges)
            .values((
                chall_dsl::category.eq(challenge.category),
                chall_dsl::ctf_id.eq(challenge_provider.get_id()),
                chall_dsl::name.eq(challenge.name),
                chall_dsl::points.eq(challenge.value),
                chall_dsl::solved.eq(false),
                chall_dsl::announced_solve.eq(false),
            ))
            .execute(&connection)?;
    }

    Ok(())
}

pub async fn remove_active_ctf(name: &str) -> Result<(), Error> {
    let connection = get_pooled_connection().await?;

    update(ctf_dsl::ctfs.filter(ctf_dsl::name.eq(name)))
        .set(ctf_dsl::active.eq(false))
        .execute(&connection)?;

    Ok(())
}

pub async fn get_ctf_id_from_name(name: &str) -> Option<i32> {
    let connection = get_pooled_connection()
        .await
        .expect("Error getting pooled connection!");

    let ctf = match ctf_dsl::ctfs
        .filter(ctf_dsl::name.eq(name))
        .limit(1)
        .load::<Ctf>(&connection)
    {
        Ok(challenges) => challenges,
        Err(why) => {
            eprintln!(
                "Error when loading challenges with name equal to {}! {}",
                name, why
            );
            vec![]
        }
    };

    Some(ctf.first()?.id)
}

fn update_working(
    update_value: Option<&str>,
    challenge_id: i32,
    connection: &MysqlConnection,
) -> Result<(), Error> {
    update(chall_dsl::challenges)
        .filter(chall_dsl::id.eq(challenge_id))
        .set(chall_dsl::working.eq(update_value))
        .execute(connection)?;
    Ok(())
}

pub fn get_challenge_from_name(
    name: &str,
    connection: &MysqlConnection,
) -> Result<Vec<Challenge>, Error> {
    let challenges = chall_dsl::challenges
        .filter(chall_dsl::name.eq(name))
        .limit(1)
        .load::<Challenge>(connection)?;
    Ok(challenges)
}

pub async fn search_for_challenge_by_name(name: &str) -> Result<Vec<Challenge>, Error> {
    let connection = get_pooled_connection().await?;
    let search = format!("%{}%", name);
    let challenges = chall_dsl::challenges
        .filter(chall_dsl::name.like(search))
        .load::<Challenge>(&connection)?;
    Ok(challenges)
}

pub async fn get_challenges_for_channel(channel_id: i64) -> Result<Vec<Challenge>, Error> {
    let connection = get_pooled_connection().await?;

    let ctf = ctf_dsl::ctfs
        .filter(ctf_dsl::channel_id.eq(channel_id))
        .limit(1)
        .load::<Ctf>(&connection)?;

    match ctf.first() {
        Some(ctf) => {
            return Ok(chall_dsl::challenges
                .filter(chall_dsl::ctf_id.eq(ctf.id))
                .load::<Challenge>(&connection)?);
        }
        None => {
            eprintln!("Error finding CTF with channel_id = {}", channel_id);
        }
    }

    return Err(format_err!("No CTF exists for that channel!"));
}

pub async fn get_challenges_for_ctfname(ctf_name: String) -> Result<Vec<Challenge>, Error> {
    let connection = get_pooled_connection().await?;

    let ctf = ctf_dsl::ctfs
        .filter(ctf_dsl::name.eq(&ctf_name))
        .limit(1)
        .load::<Ctf>(&connection)?;

    match ctf.first() {
        Some(ctf) => {
            return Ok(chall_dsl::challenges
                .filter(chall_dsl::ctf_id.eq(ctf.id))
                .load::<Challenge>(&connection)?);
        }
        None => {
            eprintln!("Error finding CTF with name = {}", ctf_name);
        }
    }

    return Err(format_err!("No CTF exists for that name!"));
}

pub async fn get_challenges_for_ctfid(ctf_id: i32) -> Result<Vec<Challenge>, Error> {
    let connection = get_pooled_connection().await?;

    let ctf = ctf_dsl::ctfs
        .filter(ctf_dsl::id.eq(&ctf_id))
        .limit(1)
        .load::<Ctf>(&connection)?;

    match ctf.first() {
        Some(ctf) => {
            return Ok(chall_dsl::challenges
                .filter(chall_dsl::ctf_id.eq(ctf.id))
                .load::<Challenge>(&connection)?);
        }
        None => {
            eprintln!("Error finding CTF with id = {}", ctf_id);
        }
    }

    return Err(format_err!("No CTF exists for that id!"));
}

pub async fn add_working(username: String, challenge_name: &str) -> Result<(), Error> {
    let connection = get_pooled_connection()
        .await
        .expect("Error when getting pooled connection");

    // First load the challenge by that name
    let challenges = get_challenge_from_name(challenge_name, &connection)?;

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
    let challenges = get_challenge_from_name(challenge_name, &connection)?;

    if let Some(challenge) = challenges.first() {
       return remove_working_from_challenge(username, &challenge, &connection);
    }

    Err(format_err!("No challenge found by that name!"))
}

pub fn remove_working_from_challenge(username: String, challenge: &Challenge, connection: &MysqlConnection) -> Result<(), Error> {
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

            return Ok(());
        }

        Err(format_err!("Unable to remove {} as working on challenge {}", username, challenge.name))
}

pub async fn mark_solved(challenge: &Challenge) -> Result<(), Error> {
    let connection = get_pooled_connection().await?;

    let challenge_id = challenge.id;

    update(chall_dsl::challenges)
        .filter(chall_dsl::id.eq(challenge_id))
        .set((
            chall_dsl::solver.eq(&challenge.solver),
            chall_dsl::solved.eq(true),
            chall_dsl::solved_time.eq(&challenge.solved_time),
            chall_dsl::announced_solve.eq(true),
        ))
        .execute(&connection)?;

        if let Some(solver) = &challenge.solver {
            return remove_working(solver.clone(), &challenge.name).await;
        }

    Ok(())
}

async fn load_active_ctfdservices() -> Result<(), Error> {
    let active_ctfs = get_active_ctfs().await?;

    // Load all active ctfs, transpose to service config and then load to cache
    for ctf in active_ctfs {
        let service_config = ChallengeProviderServiceConfig {
            name: ctf.name,
            base_url: ctf.base_url,
            api_url: ctf.api_url,
            api_key: ctf.api_key,
            service_type: ChallengeProviderServiceTypes::Ctfd, // Default as CTFd for now...
        };

        let service = new_ctfdservice(service_config).await;

        println!("Loading service into cache with id: {}", service.get_id());

        // Insert to cache
        CTF_CACHE.insert(service.get_id(), service);
    }

    Ok(())
}

pub async fn initial_load_tasks() -> Result<(), Error> {
    load_active_ctfdservices().await?;
    Ok(())
}

pub async fn check_for_new_solves(ctf: &Ctf) -> Result<Vec<Challenge>, Error> {
    if let Some(challenge_provider) = CTF_CACHE.get(&ctf.id) {
        let connection = get_pooled_connection().await?;
        let mut new_solves = vec![];
        let fresh_data = challenge_provider.get_team_solved_challenges().await?;

        for solve in fresh_data {
            let mut challenge =
                map_response_to_challenge(&connection, &solve, challenge_provider.get_id()).await?;

            if !challenge.announced_solve {
                let solved_time =
                    NaiveDateTime::parse_from_str(&solve.date, "%Y-%m-%dT%H:%M:%S%z")?;
                let solver_name = challenge_provider.user_from_id(solve.user).await?.name;

                challenge.solved_time = Some(solved_time);
                challenge.solver = Some(solver_name);

                new_solves.push(challenge);
            }
        }

        return Ok(new_solves);
    }

    Err(format_err!(
        "No challenge provider found for CTF with id: {}... Something is cooked",
        ctf.id
    ))
}

pub async fn update_challenges_and_scores(
    challenge_provider: &ChallengeProviderService,
) -> Result<(), Error> {
    let connection = get_pooled_connection().await?;

    let challenges = challenge_provider.get_challenges().await?;
    for challenge in challenges {
        let is_new = ensure_challenge_exists_otherwise_add(
            &challenge,
            challenge_provider.get_id(),
            &connection,
        )
        .await?;

        if !is_new {
            update(chall_dsl::challenges)
                .filter(chall_dsl::ctf_id.eq(challenge_provider.get_id()))
                .filter(chall_dsl::name.eq(&challenge.name))
                .filter(chall_dsl::category.eq(challenge.category))
                .filter(chall_dsl::solved.eq(false))
                .set(chall_dsl::points.eq(challenge.value))
                .execute(&connection)?;
        }
    }

    Ok(())
}

pub async fn ensure_challenge_exists_otherwise_add(
    challenge: &ChallengeResponse,
    ctf_id: i32,
    connection: &MysqlConnection,
) -> Result<bool, Error> {
    let challenges = chall_dsl::challenges
        .filter(chall_dsl::name.eq(&challenge.name))
        .limit(1)
        .load::<Challenge>(connection)?;

    if challenges.is_empty() {
        insert_into(chall_dsl::challenges)
            .values((
                chall_dsl::category.eq(&challenge.category),
                chall_dsl::ctf_id.eq(ctf_id),
                chall_dsl::name.eq(&challenge.name),
                chall_dsl::points.eq(&challenge.value),
                chall_dsl::solved.eq(false),
                chall_dsl::announced_solve.eq(false),
            ))
            .execute(connection)?;

        return Ok(true);
    }

    Ok(false)
}

pub async fn get_and_store_scoreboard(
    challenge_provider: &ChallengeProviderService,
) -> Result<(), Error> {
    let connection = get_pooled_connection().await?;
    let team_stats = challenge_provider.team_stats().await?;

    insert_into(scoreboard_dsl::scoreboard)
        .values((
            scoreboard_dsl::ctf_id.eq(challenge_provider.get_id()),
            scoreboard_dsl::points.eq(team_stats.score),
            scoreboard_dsl::position.eq(team_stats.place),
        ))
        .execute(&connection)?;

    Ok(())
}

pub async fn get_latest_scoreboard_status(ctf_id: i32) -> Result<Scoreboard, Error> {
    let connection = get_pooled_connection().await?;

    let results = scoreboard_dsl::scoreboard
        .filter(scoreboard_dsl::ctf_id.eq(ctf_id))
        .order(scoreboard_dsl::entry_id.desc())
        .limit(1)
        .load::<Scoreboard>(&connection)?;

    if let Some(scoreboard_entry) = results.first() {
        return Ok(scoreboard_entry.clone());
    }

    Err(format_err!("Unable to get latest scoreboard result"))
}

async fn map_response_to_challenge(
    connection: &PooledMysqlConnection,
    data: &TeamSolvesResponseData,
    ctf_id: i32,
) -> Result<Challenge, Error> {
    let challenges = chall_dsl::challenges
        .filter(chall_dsl::ctf_id.eq(ctf_id))
        .filter(chall_dsl::name.eq(&data.challenge.name))
        .filter(chall_dsl::category.eq(&data.challenge.category))
        .limit(1)
        .load::<Challenge>(connection)?;

    if !challenges.is_empty() {
        return Ok(challenges[0].clone());
    }

    Err(format_err!(
        "Failed to map challenge! {:#?}",
        data.challenge
    ))
}
