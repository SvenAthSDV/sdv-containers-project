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
COPY --from=builder /app/target/release/your-api-binary /usr/local/bin/your-api-binary
EXPOSE 80
CMD ["your-api-binary"]
