# Étape de développement
FROM rust:1.82.0 AS dev
WORKDIR /app
COPY . .
RUN cargo fetch
EXPOSE 80
CMD ["cargo", "run", "--release"]

# Étape de build pour la production
FROM rust:1.82.0 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Étape de production
FROM debian:buster-slim AS prod
COPY --from=builder /app/target/release/sdv-api /usr/local/bin/sdv-api
EXPOSE 80
CMD ["sdv-api"]
