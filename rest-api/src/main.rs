extern crate env_logger;

use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};

use actix_cors::Cors;
use chrono::Utc;
use ctfdb::ctfs::db::{get_active_ctfs, get_challenges_for_ctfid, get_latest_scoreboard_status};
use std::env;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ChallengeForIDResponse {
    data: Vec<ChallengeResponse>,
}

#[derive(Debug, Serialize)]
pub struct ChallengeResponse {
    title: String,
    category: String,
    status: String,
    priority: String,
    working: Option<String>,
    solver: Option<String>,
    solved: bool,
    points: i32,
    solved_time: Option<String>,
}

#[derive(Serialize)]
pub struct ActiveCTFResponse {
    data: Vec<CTFResponse>,
}

#[derive(Serialize)]
pub struct CTFResponse {
    name: String,
    id: i32,
    stats: ScoreboardResponse,
}

#[derive(Serialize)]
pub struct ScoreboardResponse {
    position: String,
    points: i32,
    entry_time: String,
}

#[get("/api/v1/active")]
async fn get_active_ctfs_route() -> impl Responder {
    return match get_active_ctfs().await {
        Ok(active_ctfs) => {
            let mut data = vec![];

            for ctf in active_ctfs {
                let scoreboard_response = match get_latest_scoreboard_status(ctf.id).await {
                    Ok(stats) => ScoreboardResponse {
                        points: stats.points,
                        position: stats.position,
                        entry_time: stats.entry_time.to_string(),
                    },
                    Err(why) => {
                        eprintln!("Error when retrieving active ctfs from database... {}", why);
                        ScoreboardResponse {
                            points: 0,
                            position: "0".to_owned(),
                            entry_time: Utc::now().naive_local().to_string(),
                        }
                    }
                };

                let ctf_response = CTFResponse {
                    name: ctf.name,
                    id: ctf.id,
                    stats: scoreboard_response,
                };

                data.push(ctf_response);
            }

            let response = ActiveCTFResponse { data };
            HttpResponse::Ok().json(response)
        }
        Err(why) => {
            eprintln!("Error when retrieving active ctfs from database... {}", why);
            HttpResponse::InternalServerError().body("Error retrieving active ctfs from database")
        }
    };
}
#[get("/api/v1/{id}/stats")]
async fn get_stats_for_id_route(web::Path(id): web::Path<i32>) -> impl Responder {
    return match get_latest_scoreboard_status(id).await {
        Ok(stats) => {
            let response = ScoreboardResponse {
                points: stats.points,
                position: stats.position,
                entry_time: stats.entry_time.to_string(),
            };

            HttpResponse::Ok().json(response)
        }
        Err(why) => {
            eprintln!(
                "Error when retrieving stats for ctf id: {} from database... {}",
                id, why
            );

            HttpResponse::InternalServerError().body(format!(
                "Error retrieving stats for ctf id {} from database",
                id
            ))
        }
    };
}

#[get("/api/v1/{id}/challenges")]
async fn get_challenges_for_id_route(web::Path(id): web::Path<i32>) -> impl Responder {
    return match get_challenges_for_ctfid(id).await {
        Ok(challenges) => {
            let mut data = vec![];

            for challenge in challenges {
                // Break down solved time to string for serialisation purposes
                let solved_time;
                if challenge.solved {
                    solved_time = Some(challenge.solved_time.unwrap().to_string());
                } else {
                    solved_time = None;
                }

                let challenge_status;
                if challenge.solved {
                    challenge_status = "DONE".to_string();
                } else if challenge.working.is_some() {
                    challenge_status = "INPROGRESS".to_string();
                } else {
                    challenge_status = "TODO".to_string();
                }

                let challenge_priority = get_challenge_priority(challenge.points);

                let challenge_response = ChallengeResponse {
                    category: challenge.category,
                    working: challenge.working,
                    solver: challenge.solver,
                    points: challenge.points,
                    solved_time,
                    title: challenge.name,
                    status: challenge_status,
                    priority: challenge_priority,
                    solved: challenge.solved,
                };

                data.push(challenge_response);
            }

            let response = ChallengeForIDResponse { data };
            HttpResponse::Ok().json(response)
        }
        Err(why) => {
            eprintln!("Error when retrieving challenges from database... {}", why);
            HttpResponse::InternalServerError().body("Error retrieving challenges from database")
        }
    };
}

fn get_challenge_priority(points: i32) -> String {
    if points < 50 {
        "LOW".to_string()
    } else if (50..100).contains(&points) {
        "MEDIUM".to_string()
    } else if (100..250).contains(&points) {
        "HIGH".to_string()
    } else {
        "HIGHEST".to_string()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let bind_address = env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8010".to_owned());

    println!("Booting up rest-api service...");

    HttpServer::new(|| {
        let allowed_origin =
            env::var("ALLOWED_ORIGIN").expect("No ALLOWED_ORIGIN environment variable set!");

        let cors = Cors::default()
            .allowed_origin(&allowed_origin)
            .allowed_methods(vec!["GET"]);
        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .service(get_challenges_for_id_route)
            .service(get_active_ctfs_route)
            .service(get_stats_for_id_route)
    })
    .bind(bind_address)?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_low_priority() {
        let expected = "LOW".to_string();
        assert_eq!(get_challenge_priority(49), expected);
        assert_eq!(get_challenge_priority(1), expected);
        assert_ne!(get_challenge_priority(50), expected);
    }

    #[test]
    fn test_medium_priority() {
        let expected = "MEDIUM".to_string();
        assert_eq!(get_challenge_priority(51), expected);
        assert_eq!(get_challenge_priority(99), expected);
        assert_ne!(get_challenge_priority(100), expected);
    }

    #[test]
    fn test_high_priority() {
        let expected = "HIGH".to_string();
        assert_eq!(get_challenge_priority(100), expected);
        assert_eq!(get_challenge_priority(249), expected);
        assert_ne!(get_challenge_priority(250), expected);
    }

    #[test]
    fn test_highest_priority() {
        let expected = "HIGHEST".to_string();
        assert_eq!(get_challenge_priority(260), expected);
        assert_eq!(get_challenge_priority(15000), expected);
    }
}
