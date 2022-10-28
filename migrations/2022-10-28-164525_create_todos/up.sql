CREATE TABLE IF NOT EXISTS todos (
    id              SERIAL PRIMARY KEY,
    title           VARCHAR NOT NULL,
    description     VARCHAR,
    status          SMALLINT NOT NULL
)