CREATE TABLE IF NOT EXISTS todos (
    id              SERIAL PRIMARY KEY,
    title           VARCHAR NOT NULL,
    description     VARCHAR,
    status          SMALLINT NOT NULL,
    tags            TEXT[]
);

CREATE TABLE IF NOT EXISTS todos_relations (
    id              SERIAL PRIMARY KEY,
    parent_todo_id  INTEGER REFERENCES todos (id),
    child_todo_id   INTEGER REFERENCES todos (id)
);