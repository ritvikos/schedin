//! CRUD Ops

extern crate sqlx;
extern crate std;
extern crate uuid;

pub mod user;
pub mod utils;

use super::error::CrudError;
use crate::job::{
    schedule::{Schedule, Time},
    schema::{Job, JobType},
};
use sqlx::{PgPool, Pool, Postgres, Transaction};
use std::sync::Arc;
use uuid::Uuid;

pub struct DB {
    pub pool: Arc<PgPool>,
    pub job: Job,
}

impl DB {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self {
            pool,
            job: Job::default(),
        }
    }

    pub fn job(mut self, job: Job) -> Self {
        self.job = job;
        self
    }

    /// Initialize Database Transaction
    pub async fn tx(&self) -> Result<Transaction<'static, Postgres>, CrudError> {
        match self.pool.begin().await {
            Ok(tx) => Ok(tx),
            Err(err) => {
                eprintln!("{}", err);
                Err(CrudError::Transaction)
            }
        }
    }

    pub async fn insert(&self, user_id: &str) -> Result<(), CrudError> {
        let user_id = Uuid::parse_str(user_id).unwrap();
        let job_id = self.job.gen_uuid();
        let job_type = self.job.kind();

        let schedule_str = self.job.schedule.as_ref().unwrap();
        let schedule = Schedule::new(schedule_str).parse().unwrap();
        let next_run = schedule.next_run();

        let (job_interval, next_run_at) = match schedule.timestamp.time {
            Time::Integer(int) => (Some(int as i32), Some(next_run)),
            Time::Timestamp(_) => (None, Some(next_run)),
        };

        let tx = self.tx().await?;

        sqlx::query!(
            r#"
            INSERT INTO jobs (user_id, job_id, job_name, job_description, job_type, job_interval, next_run_at) 
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            user_id,
            job_id,
            self.job.name,
            self.job.description,
            job_type as JobType,
            job_interval,
            next_run_at
        )
        .execute(&*self.pool)
        .await
        .unwrap();

        if let Some(task) = &self.job.task {
            sqlx::query!(
                r#"
                INSERT INTO tasks (job_id, task_name) 
                VALUES ($1, $2)
                "#,
                job_id,
                task.name
            )
            .execute(&*self.pool)
            .await
            .unwrap();
        };

        if let Some(code) = &self.job.code {
            sqlx::query!(
                r#"
                INSERT INTO codes (job_id, src, lang, cmd) 
                VALUES ($1, $2, $3, $4)
                "#,
                job_id,
                code.src,
                code.lang,
                code.cmd
            )
            .execute(&*self.pool)
            .await
            .unwrap();
        };

        if let Some(bin) = &self.job.bin {
            sqlx::query!(
                r#"
                INSERT INTO bins (job_id, path, cmd) 
                VALUES ($1, $2, $3)
                "#,
                job_id,
                bin.path,
                bin.cmd
            )
            .execute(&*self.pool)
            .await
            .unwrap();
        };

        if let Err(err) = tx.commit().await {
            eprintln!("Failed to commit transaction: {}", err);
            return Err(CrudError::Transaction);
        }

        Ok(())
    }

    pub async fn delete(&self, user_id: String) -> Result<(), CrudError> {
        let user_id = Uuid::parse_str(&user_id).unwrap();

        let tx = self.tx().await?;

        sqlx::query!(
            r#"
            DELETE FROM jobs WHERE user_id=$1 AND job_name=$2
            "#,
            user_id,
            self.job.name
        )
        .execute(&*self.pool)
        .await
        .unwrap();

        if let Err(err) = tx.commit().await {
            eprintln!("Failed to commit transaction: {}", err);
            return Err(CrudError::Transaction);
        }

        Ok(())
    }
}
