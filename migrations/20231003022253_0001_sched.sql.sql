CREATE TYPE job_type AS ENUM ('Bin', 'Code', 'Task');

CREATE TYPE job_status AS ENUM (
    'Running',
    'Paused',
    'Scheduled',
    'Disabled'
);

CREATE TABLE task (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

CREATE TABLE code (
    id UUID PRIMARY KEY,
    src TEXT NOT NULL,
    lang VARCHAR(50),
    cmd VARCHAR(255)
);

CREATE TABLE bin (
    id UUID PRIMARY KEY,
    path VARCHAR(255) NOT NULL,
    cmd VARCHAR(255)
);

CREATE TABLE IF NOT EXISTS jobs (
    id UUID PRIMARY KEY,
    type job_type,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    runs INTEGER,
    schedule VARCHAR(255),
    error_count INTEGER,
    next_run_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    status job_status NOT NULL,
    task UUID REFERENCES task(id),
    bin UUID REFERENCES bin(id),
    code_id UUID REFERENCES code(id)
);

-- index on the 'name' column for faster name-based lookups
CREATE INDEX idx_jobs_name ON jobs(name);

-- index on the 'status' column for faster status-based lookups
CREATE INDEX idx_jobs_status ON jobs(status);

-- index on the 'created_at' column for faster sorting
CREATE INDEX idx_jobs_created_at ON jobs(created_at);