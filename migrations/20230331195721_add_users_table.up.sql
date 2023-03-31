CREATE TABLE IF NOT EXISTS users (
    id UUID NOT NULL,
    first_name VARCHAR,
    last_name VARCHAR,
    username VARCHAR(16) NOT NULL UNIQUE,
    email VARCHAR NOT NULL UNIQUE,
    pwd_hash VARCHAR NOT NULL,
    age INTEGER,
    about VARCHAR(512),
    verified BOOLEAN NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (id)
);
