FROM docker.io/library/rust:1.71.0-bookworm AS builder
LABEL authors="jason.tulp"

RUN rustup show && cargo run

EXPOSE 2048 8080
ENTRYPOINT ["top", "-b"]