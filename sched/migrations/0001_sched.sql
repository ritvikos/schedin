CREATE TYPE job_type AS ENUM ('Binary', 'Code', 'Task');

CREATE TYPE job_status AS ENUM (
    'Running',
    'Paused',
    'Scheduled',
    'Disabled'
);

CREATE TABLE IF NOT EXISTS jobs (
    id VARCHAR(255) PRIMARY KEY,
    type job_type,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    runs INTEGER,
    schedule VARCHAR(255),
    error_count INTEGER,
    next_run_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    status job_status NOT NULL,
    task VARCHAR(255),
    code TEXT,
    store VARCHAR(255),
    CONSTRAINT enforce_type CHECK (
        (
            type = 'Binary'
            AND store IS NOT NULL
        )
        OR (
            type = 'Code'
            AND code IS NOT NULL
        )
        OR (
            type = 'Task'
            AND task IS NOT NULL
        )
    )
);

-- index on the 'name' column for faster name-based lookups
CREATE INDEX idx_jobs_name ON jobs(name);

-- index on the 'status' column for faster status-based lookups
CREATE INDEX idx_jobs_status ON jobs(status);

-- index on the 'created_at' column for faster sorting
CREATE INDEX idx_jobs_created_at ON jobs(created_at);