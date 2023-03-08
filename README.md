# 1. Intro

This is just an experimental project to explore building a web service or API in Rust. It uses the powerful [Actix Web](https://actix.rs/) framework.

# 2. How to run?

Easiest way to spin up a working sample of this repo is via Docker Compose

## 2.1 Docker Compose

To spin up the entire stack on local, just run the following command (depending on your target architecture).

For arm64 / aarch64 / M1 Mac for example, it is supported by default

```
docker-compose up
```

For amd84 / x86_64 / Intel chips for example,

```
docker-compose build --build-arg "ARCH=x86_64"
docker-compose up
```

You need to have [Docker](https://www.docker.com/products/docker-desktop/) installed on your machine.

The components included in the compose file are

- API (Actix Web)
- Database (Postgres)
- Monitoring - metrics (Jaeger)

## 2.2 Diesel

You will also need to install some dependencies required to work with [Diesel](https://diesel.rs/), i.e. the ORM used in this project. You can refer to the [notes.md](https://github.com/DriLLFreAK100/codefee-works-api/blob/main/notes.md) file.

## 2.3 Running the DB Migrations

To setup the database structure required by the API, you need to run the DB migration files. It is located in this [directory](https://github.com/DriLLFreAK100/codefee-works-api/tree/main/migrations).

The easiest way to run the migration here is by using the `Diesel CLI` with the following command

```
diesel migration run
```

For more details, you can refer to the [notes.md](https://github.com/DriLLFreAK100/codefee-works-api/blob/main/notes.md) file.

## 2.4 `.env` file

You can just make a copy from `.env.local` (the sample env file) and rename it as `.env`.

## 2.5 For Local Development

To develop the Actix Web application itself, you need to first stop the `core-api` service if you did spun it up using docker-compose in the previous steps.

After that, you can start the application via the following command

```
cargo run
```

It works just like any other Rust applications that use [Cargo](https://doc.rust-lang.org/cargo/).

# 3. OpenAPI (Swagger)

This project has been setup with [utoipa](https://github.com/juhaku/utoipa) to support OpenAPI doc generations.

After spinning up the API following the steps above, you can visit the Swagger UI at `http://127.0.0.1:8080/swagger-ui/#`.

It will only be available when the env variable `RUN_ENV=dev`.

# 4. About the Dockerfile

I have written an article that explains about the `Dockerfile` used in this project. It contains some information about running an Actix Web application that uses [Diesel](https://diesel.rs/), as well as some steps to optimize Docker image size as well. You can check it out here on [Codefee Time](https://www.codefeetime.com/post/docker-config-for-actix-web-diesel-and-postgres).

# 5. Things to do

| Items                                                                                     | Status               |
| ----------------------------------------------------------------------------------------- | -------------------- |
| Actix Routes                                                                              | :white_check_mark:   |
| Error Handling                                                                            | :white_check_mark:   |
| Containerization                                                                          | :white_check_mark:   |
| Observability - tracing with [jaeger](https://www.jaegertracing.io/docs/1.41/monitoring/) | :white_check_mark:   |
| OpenAPI - with [utoipa](https://github.com/juhaku/utoipa)                                 | :white_check_mark:   |
| Auth                                                                                      | :white_large_square: |
| ?                                                                                         | :white_large_square: |
