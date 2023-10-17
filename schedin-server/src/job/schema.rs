//! Job Schema
//! Unified Schema for API and Database

extern crate serde;
extern crate uuid;
extern crate validator;

use crate::api::validation::{validate_schedule, validate_source_format};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Default, Deserialize, Validate)]
pub struct Job {
    pub name: String,
    pub description: Option<String>,
    #[validate(custom(
        function = "validate_schedule",
        message = "Valid Examples: @every 10 min or @once 2023-10-17 06:45:00>"
    ))]
    pub schedule: Option<String>,
    pub task: Option<Task>,
    #[validate]
    pub code: Option<Code>,
    pub bin: Option<Bin>,
}

impl Job {
    /// Generate UUID v4
    pub fn gen_uuid(&self) -> Uuid {
        Uuid::new_v4()
    }

    /// Get `JobType` enum
    pub fn kind(&self) -> JobType {
        if self.task.is_some() {
            return JobType::Task;
        } else if self.code.is_some() {
            return JobType::Code;
        } else if self.bin.is_some() {
            return JobType::Bin;
        }
        JobType::Invalid
    }
}

#[derive(Debug, Deserialize, sqlx::types::Type)]
#[sqlx(type_name = "job_types", rename_all = "lowercase")]
pub enum JobType {
    Bin,
    Code,
    Invalid,
    Task,
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

#[derive(Debug, Deserialize, sqlx::types::Type)]
#[sqlx(type_name = "job_status", rename_all = "lowercase")]
pub enum JobStatus {
    /// Job is running
    Running,

    /// Job is scheduled
    Scheduled,

    /// Job is disabled
    Disabled,
}

// Bin

#[derive(Debug, Default, Deserialize)]
pub struct Bin {
    pub path: String,
    pub cmd: Option<String>,
}

// Code

#[derive(Debug, Default, Deserialize, Validate)]
pub struct Code {
    #[validate(custom(
        function = "validate_source_format",
        message = "Source Code must be base64-encoded"
    ))]
    pub src: String,
    pub lang: String,
    pub cmd: String,
}

// Task

#[derive(Debug, Default, Deserialize)]
pub struct Task {
    pub name: String,
}
