#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;

use async_trait::async_trait;
use chrono::{DateTime, Local};
use ctfs::structs::{
    ChallengeResponse, MyTeamResponseData, TeamSolvesResponseData, UserResponseData,
};
use htb::structs::JWTClaims;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, ClientBuilder,
};

use std::{env, sync::Arc, time::Duration};

use async_rwlock::RwLock;
use diesel::{
    r2d2::{self, ConnectionManager},
    MysqlConnection,
};
use failure::Error;
use once_cell::sync::Lazy;

use crate::r2d2::PooledConnection;

pub mod ctfs;
pub mod htb;
pub mod models;
pub mod schema;

type MysqlConnectionPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;
type PooledMysqlConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

static DB: Lazy<Arc<RwLock<MysqlConnectionPool>>> = Lazy::new(|| {
    let db_url = env::var("DATABASE_URL").expect("No DATABASE_URL environment variable defined!");
    let manager = ConnectionManager::<MysqlConnection>::new(db_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let rwlock = RwLock::new(pool);
    Arc::new(rwlock)
});

async fn get_pooled_connection() -> Result<PooledMysqlConnection, Error> {
    let lock = &DB.read().await;
    let connection = lock.get()?;
    Ok(connection)
}

pub fn create_reqwest_client(api_key: &str, token_type: &str) -> Client {
    let mut headers = HeaderMap::new();

    let auth_header = HeaderValue::from_str(&format!("{} {}", token_type, &api_key))
        .expect("Error creating auth header for new ctfd service");

    let content_type_header = HeaderValue::from_str("application/json")
        .expect("Error when creating content type header for new htb api instance");

    headers.insert("Authorization", auth_header);
    headers.insert("Content-Type", content_type_header);

    ClientBuilder::new()
        .timeout(Duration::from_secs(5))
        .cookie_store(true)
        .default_headers(headers)
        .build()
        .expect("Error when creating reqwest client")
}

#[async_trait]
pub trait ChallengeProvider {
    fn get_id(&self) -> i32;
    async fn get_challenges(&self) -> Result<Vec<ChallengeResponse>, Error>;
    async fn get_team_solved_challenges(&self) -> Result<Vec<TeamSolvesResponseData>, Error>;
    async fn user_from_id(&self, id: i32) -> Result<UserResponseData, Error>;
    async fn team_stats(&self) -> Result<MyTeamResponseData, Error>;
}

pub fn jwt_still_valid(jwt: &JWTClaims) -> bool {
    let local: DateTime<Local> = Local::now();
    let unix_epoch = local.timestamp();

    if unix_epoch > jwt.exp {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_jwt_validity() {
        let local: DateTime<Local> = Local::now();

        let tomorrow_midnight = (local + Duration::days(1)).date().and_hms(0, 0, 0);
        let yesterday_midnight = (local - Duration::days(1)).date().and_hms(0, 0, 0);

        let jwt = JWTClaims {
            exp: tomorrow_midnight.timestamp(),
        };

        assert!(jwt_still_valid(&jwt));

        let jwt = JWTClaims {
            exp: yesterday_midnight.timestamp(),
        };

        assert!(!jwt_still_valid(&jwt));
    }
}
