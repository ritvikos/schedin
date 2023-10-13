//! Database Operations

extern crate sqlx;

use crate::{
    error::CrudError,
    job::{Job, JobStatus, JobType},
};
use sqlx::{Database, Pool, Postgres, Transaction};

pub struct DB {
    pub pool: Pool<Postgres>,
    pub job: Job,
}

impl DB {
    /// Create new instance
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self {
            pool,
            job: Job::new(),
        }
    }

    /// Initialize Database Transaction
    async fn tx(&self) -> Result<Transaction<'static, Postgres>, CrudError> {
        match self.pool.begin().await {
            Ok(tx) => Ok(tx),
            Err(err) => {
                eprintln!("{}", err);
                Err(CrudError::Transaction)
            }
        }
    }

    /// Read All Database Records
    pub async fn read_all(&self) -> Result<Vec<Job>, CrudError> {
        let tx = self.tx().await?;

        let jobs = sqlx::query_as!(
            Job,
            r#"
        SELECT user_id, job_id, job_name, job_description, 
        job_type as "job_type: JobType", schedule, runs, error_count, next_run_at, 
        created_at, job_status as "job_status: JobStatus" 
        from jobs"#
        )
        .fetch_all(&self.pool)
        .await
        .unwrap();

        if let Err(err) = tx.commit().await {
            eprintln!("{}", err);
            return Err(CrudError::Transaction);
        }

        Ok(jobs)
    }
}

/// Create Database Connection Pool
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
