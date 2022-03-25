DROP TABLE IF EXISTS credentials;
DROP TABLE IF EXISTS users;

CREATE TABLE IF NOT EXISTS users (
    uuid UUID NOT NULL,
    username TEXT UNIQUE NOT NULL,
    about TEXT,
    join_date TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY(uuid)
);

CREATE TABLE IF NOT EXISTS credentials (
    id INT GENERATED ALWAYS AS IDENTITY NOT NULL,
    owner_uuid UUID NOT NULL REFERENCES users(uuid) ON DELETE CASCADE,
    email TEXT UNIQUE NOT NULL,
    is_email_confirmed BOOLEAN NOT NULL DEFAULT false,
    pwd_hash TEXT NOT NULL,
    PRIMARY KEY(id)
);

