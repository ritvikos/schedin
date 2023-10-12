//! DB Utilities

extern crate sqlx;
extern crate uuid;

use crate::error::CrudError;
use sqlx::{PgPool, Pool, Postgres};

pub async fn create_pool(database_url: &str) -> Result<Pool<Postgres>, CrudError> {
    match PgPool::connect(database_url).await {
        Ok(pool) => Ok(pool),
        Err(err) => {
            eprintln!("{}", err);
            Err(CrudError::Pooling)
        }
    }
}
