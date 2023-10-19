//! CRUD Ops

extern crate schedin_common;
extern crate sqlx;
extern crate std;
extern crate uuid;

pub mod user;

use crate::job::{
    schedule::{Schedule, Time},
    schema::{Bin, Code, Job, JobType, Task},
};
use schedin_common::{error::CrudError, tx::Tx};
use sqlx::{PgPool, Pool, Postgres};
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

    pub async fn insert(&self, user_id: &str) -> Result<(), CrudError> {
        let tx_manager = Tx::new(self.pool.clone());
        let tx = tx_manager.init().await?;

        let job_id = match self.insert_inner(user_id).await {
            Ok(id) => id,
            Err(error) => {
                tx_manager.rollback(tx).await?;
                return Err(error);
            }
        };

        if let Some(task) = &self.job.task {
            self.task(&job_id, task).await?;
        };

        if let Some(code) = &self.job.code {
            self.code(&job_id, code).await?;
        };

        if let Some(bin) = &self.job.bin {
            self.bin(&job_id, bin).await?;
        };

        tx_manager.commit(tx).await?;

        Ok(())
    }

    async fn insert_inner(&self, user_id: &str) -> Result<Uuid, CrudError> {
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

        match sqlx::query!(
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
        .await {
            Ok(_) => Ok(job_id),
            Err(e) => {
                eprintln!("{}", e);
                Err(CrudError::Insertion)
            }
        }
    }

    async fn task(&self, job_id: &Uuid, task: &Task) -> Result<(), CrudError> {
        match sqlx::query!(
            r#"
            INSERT INTO tasks (job_id, task_name) 
            VALUES ($1, $2)
            "#,
            job_id,
            task.name
        )
        .execute(&*self.pool)
        .await
        {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("{}", e);
                Err(CrudError::Insertion)
            }
        }
    }

    async fn code(&self, job_id: &Uuid, code: &Code) -> Result<(), CrudError> {
        match sqlx::query!(
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
        {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("{}", e);
                Err(CrudError::Insertion)
            }
        }
    }

    async fn bin(&self, job_id: &Uuid, bin: &Bin) -> Result<(), CrudError> {
        match sqlx::query!(
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
        {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("{}", e);
                Err(CrudError::Insertion)
            }
        }
    }

    pub async fn delete(&self, user_id: String) -> Result<(), CrudError> {
        let user_id = Uuid::parse_str(&user_id).unwrap();

        let tx_manager = Tx::new(self.pool.clone());
        let tx = tx_manager.init().await?;

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

        tx_manager.commit(tx).await?;

        Ok(())
    }
}
