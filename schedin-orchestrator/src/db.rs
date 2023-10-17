//! Database Operations

extern crate sqlx;
extern crate std;

use crate::{
    error::CrudError,
    job::{Job, JobStatus, JobType},
};
use sqlx::{types::time::OffsetDateTime, Database, Pool, Postgres, Transaction};
use std::time::Duration;

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

    /// # Read
    /// Read Database Records containing scheduled jobs within a specified time range.
    ///
    /// This function retrieves a list of scheduled jobs from a database that fall within
    /// a specified time range starting from the current time. The range is defined by the
    /// `range` parameter, which represents the maximum time difference, in seconds,
    /// from the current time for a job to be considered within range.
    ///
    /// ## Arguments
    ///
    /// `interval` - The time range duration from the current time.
    /// Jobs falling within this range from the current time will
    /// be included in the result.
    ///
    /// ## Returns
    ///
    /// A `Result` containing a vector of `Job` structures representing the scheduled jobs
    /// found within the specified time range. If the operation is successful, it returns
    /// `Ok(Vec<Job>)`. If an error occurs during the database read operation, it returns
    /// `Err(CrudError)` with details about the error.
    ///
    /// ## Errors
    ///
    /// `Err(CrudError)` provides information about the specific error that occurred.
    pub async fn read(&self, interval: Duration) -> Result<Vec<Job>, CrudError> {
        let tx = self.tx().await?;

        let current_time = OffsetDateTime::now_utc();
        let interval = current_time + interval;

        println!("{:?}", current_time);
        println!("{:?}", interval);

        let jobs = sqlx::query_as!(
            Job,
            r#"
            SELECT user_id, job_id, job_name, job_description, 
            job_type as "job_type: JobType", schedule, runs, error_count, next_run_at, 
            created_at, job_status as "job_status: JobStatus" FROM jobs 
            WHERE next_run_at BETWEEN $1 AND $2 
            AND job_status = 'scheduled';
            "#,
            current_time,
            interval
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
