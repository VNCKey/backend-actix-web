# ---- Builder ----
FROM rust:latest as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY . .

RUN cargo build --release

# ---- Runtime ----
FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && update-ca-certificates

# Copiar binario
COPY --from=builder /app/target/release/learning_rust /app/app

# Copiar config
COPY config/default.toml /app/config/default.toml

EXPOSE 8080

CMD ["./app"]
