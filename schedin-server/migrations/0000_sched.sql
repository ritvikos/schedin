CREATE TABLE IF NOT EXISTS users (
    user_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(255) UNIQUE NOT NULL,
    passcode VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TYPE job_types AS ENUM ('bin', 'code', 'task');

CREATE TYPE job_status AS ENUM (
    'running',
    'paused',
    'scheduled',
    'disabled'
);

CREATE TABLE IF NOT EXISTS jobs (
    user_id UUID REFERENCES users(user_id) ON DELETE CASCADE,
    job_id UUID PRIMARY KEY,
    job_name VARCHAR(255) NOT NULL,
    job_description TEXT,
    job_type job_types NOT NULL,
    schedule VARCHAR(255),
    runs INTEGER DEFAULT 0,
    error_count INTEGER DEFAULT 0,
    next_run_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    job_status job_status DEFAULT 'scheduled' NOT NULL,
    CONSTRAINT unique_job_name_per_user UNIQUE (user_id, job_name)
);

CREATE TABLE IF NOT EXISTS tasks (
    task_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    job_id UUID REFERENCES jobs(job_id) ON DELETE CASCADE,
    task_name VARCHAR(255) NOT NULL,
    segment VARCHAR(255)
);

CREATE TABLE IF NOT EXISTS codes (
    code_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    job_id UUID REFERENCES jobs(job_id) ON DELETE CASCADE,
    src TEXT NOT NULL,
    lang VARCHAR(50) NOT NULL,
    cmd VARCHAR(255) NOT NULL,
    segment VARCHAR(255)
);

CREATE TABLE IF NOT EXISTS bins (
    bin_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    job_id UUID REFERENCES jobs(job_id) ON DELETE CASCADE,
    path VARCHAR(255) NOT NULL,
    cmd VARCHAR(255),
    segment VARCHAR(255)
);

-- index on the 'id' for faster id-based lookups
CREATE INDEX idx_jobs_id ON users(user_id);

-- index on the 'name' column for faster name-based lookups
CREATE INDEX idx_jobs_name ON jobs(job_name);

-- index on the 'job_status' column for faster status-based lookups
CREATE INDEX idx_jobs_status ON jobs(job_status);

-- index on the 'created_at' column for faster sorting
CREATE INDEX idx_jobs_created_at ON jobs(created_at);