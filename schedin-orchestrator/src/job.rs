//! Job

extern crate sqlx;

use sqlx::types::{time::OffsetDateTime, Uuid};

#[derive(Debug, Default, sqlx::FromRow)]
pub struct Job {
    pub user_id: Option<Uuid>,
    pub job_id: Option<Uuid>,
    pub job_name: Option<String>,
    pub job_description: Option<String>,
    pub job_type: JobType,
    pub schedule: String,
    pub runs: Option<i32>,
    pub error_count: Option<i32>,
    pub next_run_at: Option<OffsetDateTime>,
    pub created_at: Option<OffsetDateTime>,
    pub job_status: JobStatus,
}

impl Job {
    pub fn new() -> Self {
        Self::default()
    }
}

// Job Type
#[derive(Debug, sqlx::types::Type)]
#[sqlx(type_name = "job_types", rename_all = "lowercase")]
pub enum JobType {
    Bin,
    Code,
    Invalid,
    Task,
}

impl Default for JobType {
    fn default() -> Self {
        Self::Invalid
    }
}

impl ToString for JobType {
    fn to_string(&self) -> String {
        match *self {
            JobType::Bin => "bin".to_string(),
            JobType::Code => "code".into(),
            JobType::Invalid => "invalid".into(),
            JobType::Task => "task".into(),
        }
    }
}

// Job Status
#[derive(Debug, sqlx::types::Type)]
#[sqlx(type_name = "job_status", rename_all = "lowercase")]
pub enum JobStatus {
    /// Job is running
    Running,

    /// Job is scheduled
    Scheduled,

    /// Job is disabled
    Disabled,
}

impl Default for JobStatus {
    fn default() -> Self {
        Self::Disabled
    }
}

// Bin
#[derive(Debug, Default)]
pub struct Bin {
    pub path: String,
    pub cmd: Option<String>,
}

// Code
#[derive(Debug, Default)]
pub struct Code {
    pub src: String,
    pub lang: String,
    pub cmd: String,
}

// Task
#[derive(Debug, Default, sqlx::FromRow)]
pub struct Task {
    pub name: String,
}
