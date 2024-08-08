# Use the official Rust image as a base
FROM rust:1-slim-bookworm AS build

ARG pkg=rocket-starter

# Create a new directory for the application code
WORKDIR /build

# Copy the Cargo.toml and Cargo.lock files to the container
COPY . .

# Build the application
RUN --mount=type=cache,target=/build/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    set -eux; \
    cargo build --release; \
    objcopy --compress-debug-sections target/release/$pkg ./main

# Use a minimal base image for the final container
FROM debian:bookworm-slim

WORKDIR /app

# Install PostgreSQL client
RUN apt-get update && \
    apt-get install -y \
    postgresql postgresql-contrib \
    && rm -rf /var/lib/apt/lists/*

ENV POSTGRES_USER=postgres
ENV POSTGRES_PASSWORD=1234
ENV POSTGRES_DB=rocket_starter

RUN service postgresql start && \
    su -c "psql -c \"ALTER USER ${POSTGRES_USER} PASSWORD '${POSTGRES_PASSWORD}';\"" postgres && \
    su -c "psql -c \"CREATE DATABASE ${POSTGRES_DB};\"" postgres && \
    su -c "psql -c \"GRANT ALL PRIVILEGES ON DATABASE ${POSTGRES_DB} TO ${POSTGRES_USER};\"" postgres


# Copy the main binary
COPY --from=build /build/main .

# copy runtime assets which may or may not exist
COPY --from=build /build/Rocket.toml .

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080

CMD ["service", "postgresql", "start", "&&", "./main"]