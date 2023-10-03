//! Job

pub mod db;

pub type TimeStamp = String;

pub struct Job {
    id: String,
    name: String,
    description: String,
    job_type: JobType,
    task: String,
    code: Option<Code>,
    bin_path: Option<Bin>,
    schedule: TimeStamp,
    next_run_at: TimeStamp,
}

pub enum JobType {
    Bin,
    Code,
    Task,
}

pub struct Code {
    code: String,
    lang: String,
    cmd: String,
}

pub struct Bin {
    path: String,
    cmd: Option<String>,
}
