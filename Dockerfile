FROM lukemathwalker/cargo-chef:latest-rust-1.90.0 AS chef

WORKDIR /app

RUN apt-get update -y && apt-get install -y clang

FROM chef AS planner

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

ENV SQLX_OFFLINE=true

RUN cargo build --release --bin ztp

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
