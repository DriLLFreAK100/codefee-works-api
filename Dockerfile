FROM rust:slim-buster

RUN apt-get update && \
  apt-get -y upgrade && \
  apt-get -y install libpq-dev

WORKDIR /app
COPY . /app/
COPY .env.docker /app/.env

RUN cargo build --release

EXPOSE 8080

ENTRYPOINT ["/bin/bash", "-c", "cargo run --release"]