# Stage 1 ------------------------------------------

FROM rust:1.87-slim-bullseye AS chef

RUN cargo install cargo-chef

WORKDIR /app
# -----------------------------------------


# Stage 2 ------------------------------------------

FROM chef AS planner

COPY . .

RUN cargo chef prepare  --recipe-path recipe.json
# -----------------------------------------


# Stage 3 ------------------------------------------ 

FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY . .

ENV SQLX_OFFLINE=true

RUN cargo build --release
# -----------------------------------------


# Stage 4 ------------------------------------------

FROM debian:bullseye-slim

COPY --from=builder /app/target/release/axum_askama_tutorial /usr/local/bin

# EXPOSE 9090

ENTRYPOINT [ "/usr/local/bin/axum_askama_tutorial" ]
# -----------------------------------------    