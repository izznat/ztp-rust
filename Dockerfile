FROM rust:1.90.0 AS builder

WORKDIR /app

RUN apt update && apt install clang -y

COPY . .

ENV SQLX_OFFLINE=true

RUN cargo build --release

FROM debian:trixie-slim AS runtime

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/ztp ztp

COPY configuration configuration

ENTRYPOINT [ "./ztp" ]
