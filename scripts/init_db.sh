#!/usr/bin/env bash

if [ ! -f ".env" ]; then
    echo >&2 "File .env not found"
    exit
else
    source .env
fi

docker create \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -p "${DB_PORT}":5432 \
    --name "pg_blog_db" \
    postgres -N 1000

docker start $(docker ps --all --filter "name=pg_blog_db" --format "{{.ID}}")

sleep 5

sqlx database create
sqlx migrate run

