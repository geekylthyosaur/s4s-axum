CREATE TABLE IF NOT EXISTS orders (
    id UUID NOT NULL,
    student_uuid UUID NOT NULL,
    mentor_uuid UUID NOT NULL,
    price INTEGER NOT NULL,
    title VARCHAR(128) NOT NULL,
    description VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    PRIMARY KEY(id)
);
