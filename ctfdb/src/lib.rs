#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;

use std::{env, sync::Arc};

use async_rwlock::RwLock;
use diesel::{
    r2d2::{self, ConnectionManager},
    MysqlConnection,
};
use failure::Error;
use once_cell::sync::Lazy;

use crate::r2d2::PooledConnection;

pub mod ctfd;
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
