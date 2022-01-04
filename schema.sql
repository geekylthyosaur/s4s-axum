DROP TABLE IF EXISTS posts;
DROP TABLE IF EXISTS users;

CREATE TABLE IF NOT EXISTS posts (
    id SERIAL PRIMARY KEY NOT NULL,
    uuid UUID NOT NULL,
    owner_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    date_created TIMESTAMPTZ NOT NULL DEFAULT now(),
    date_updated TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY NOT NULL,
    uuid UUID NOT NULL,
    username TEXT UNIQUE NOT NULL,
    email TEXT UNIQUE NOT NULL,
    passwd_hash TEXT NOT NULL,
    about TEXT,
    join_date TIMESTAMPTZ NOT NULL DEFAULT now()
);
