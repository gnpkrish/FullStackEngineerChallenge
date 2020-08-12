# build stage
FROM rust:slim as build

# install libpq and create new empty binary project
RUN apt-get update; \
    apt-get install --no-install-recommends -y libpq-dev; \
    rm -rf /var/lib/apt/lists/*; \
    USER=root cargo new --bin app
WORKDIR /app

# copy manifests
COPY ./backend/Cargo.toml ./Cargo.toml
RUN mkdir .cargo
RUN cargo vendor > .cargo/config

# build this project to cache dependencies
RUN cargo build --release; \
    rm src/*.rs

# copy project source and necessary files
COPY ./backend/src ./src
COPY ./backend/migrations ./migrations
COPY ./backend/keys ./keys
COPY ./backend/diesel.toml .
COPY ./backend/.env .

# add .env and secret.key for Docker env
RUN touch .env

# rebuild app with project source
RUN rm ./target/release/deps/ers*; \
    cargo build --release

# deploy stage
FROM debian:buster-slim

# create app directory
RUN mkdir app
WORKDIR /app

# install libpq
RUN apt-get update; \
    apt-get install --no-install-recommends -y libpq-dev; \
    rm -rf /var/lib/apt/lists/*

# copy binary and configuration files
COPY --from=build /app/target/release/ers .
COPY --from=build /app/.env .
COPY --from=build /app/diesel.toml .

ENV WAIT_VERSION 2.7.2
ADD https://github.com/ufoscout/docker-compose-wait/releases/download/$WAIT_VERSION/wait /wait
RUN chmod +x /wait
# expose port
EXPOSE 8000

# run the binary
CMD ["/app/ers"]
