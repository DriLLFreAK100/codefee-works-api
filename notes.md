# To Setup Diesel (MacOS)

1. brew install libpq && brew link --force libpq
2. brew install lpq
3. cargo install diesel_cli

Refer to https://diesel.rs/guides/getting-started

## To run migration (diesel)

1. Generate migration files `diesel migration generate your_migration_name`
2. Fill up the `up.sql` and `down.sql` for updates and rollbacks respectively
3. Perform migration `diesel migration run`
4. Rerun migration (clean run) `diesel migration redo`

## To run docker-compose

1. `docker compose up` to spin up everything
2. `docker compose up --build` to build + spin up everything
3. `docker compose up [service_name]` to spin up specific service
4. `docker compose up --build [service_name]` to build and spin up specific service
