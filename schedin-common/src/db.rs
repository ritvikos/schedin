//! Database Utils

extern crate sqlx;

use crate::error::CrudError;
use sqlx::{Database, Pool};

/// # Create Pool
/// Create database connection pool
pub async fn create_pool<D>(conn_str: &str) -> Result<Pool<D>, CrudError>
where
    D: Database,
{
    match Pool::<D>::connect(conn_str).await {
        Ok(pool) => Ok(pool),
        Err(err) => {
            eprintln!("{}", err);
            Err(CrudError::Pooling)
        }
    }
}
