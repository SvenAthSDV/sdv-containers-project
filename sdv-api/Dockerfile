FROM rust:1.82.0
WORKDIR /app
COPY . .
RUN cargo fetch
RUN cargo build --release
EXPOSE 80
CMD ["cargo", "run", "--release"]