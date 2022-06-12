FROM rust as builder

ENV PACKAGE=acme

ADD .. /seed
WORKDIR /seed

COPY .. .
RUN cargo build acme --release && cargo build acme-macros --release
RUN cargo test --package acme --test standard