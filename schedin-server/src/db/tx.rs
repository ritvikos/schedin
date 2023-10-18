//! Database Transaction

extern crate sqlx;
extern crate std;

use crate::error::CrudError;
use sqlx::{Database, Pool, Transaction};
use std::sync::Arc;

/// Database Transaction
pub struct Tx<D: Database>(Arc<Pool<D>>);

impl<D> Tx<D>
where
    D: Database,
{
    /// New Instance
    pub fn new(pool: Arc<Pool<D>>) -> Self {
        Self(pool)
    }

    /// Initialize Database Transaction
    pub async fn init(&self) -> Result<Transaction<'_, D>, CrudError> {
        match self.0.begin().await {
            Ok(tx) => Ok(tx),
            Err(err) => {
                eprintln!("{}", err);
                Err(CrudError::Transaction)
            }
        }
    }

    /// Commit Database Transaction
    pub async fn commit(&self, tx: Transaction<'_, D>) -> Result<(), CrudError> {
        tx.commit().await.map_err(|e| {
            eprintln!("Failed to commit transaction: {}", e);
            CrudError::Transaction
        })?;
        Ok(())
    }

    /// Rollback Database Transaction
    pub async fn rollback(&self, tx: Transaction<'_, D>) -> Result<(), CrudError> {
        tx.rollback().await.map_err(|e| {
            eprintln!("Failed to rollback transaction: {}", e);
            CrudError::Transaction
        })?;
        Ok(())
    }
}
