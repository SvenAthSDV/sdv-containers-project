# Dockerfile pour l'environnement de développement
FROM rust:1.82.0 AS dev
WORKDIR /app
COPY . .
RUN cargo fetch
RUN cargo build --release
EXPOSE 80
CMD ["cargo", "run", "--release"]
