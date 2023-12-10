FROM rust:latest

RUN rustup component add clippy
RUN rustup component add rustfmt
RUN cargo install cargo-generate just
RUN cargo install cargo-nextest --locked