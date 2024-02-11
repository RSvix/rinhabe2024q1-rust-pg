# Build
FROM rust:1.76 as builder
RUN apt update -y && apt install -y pkg-config libssl-dev
WORKDIR /app
RUN mkdir src; touch src/main.rs
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch
COPY src/ ./src/
RUN cargo build --release

# Runtime
FROM debian:bookworm-slim
RUN apt update -yqq && apt install -yqq pkg-config libssl-dev
WORKDIR /app
COPY --from=builder app/target/release/rinha2024q1rust ./
EXPOSE 8080
CMD ["./rinha2024q1rust"]

# docker build -t rinhateste .
# docker run rinhateste:latest