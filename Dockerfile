FROM ubuntu as builder-base

RUN apt-get update -y
RUN apt-get upgrade -y

RUN apt-get install -y \
    apt-utils \
    build-essential  \
    cmake \
    curl \
    pkg-config

FROM builder-base as environment

RUN apt-get install -y \
    libpcsclite1

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

FROM environment as project

ADD . /project
WORKDIR /project

COPY . .
RUN cargo fmt --all && \
    cargo build --workspace --release --verbose --color always && \
    cargo test --all-features --verbose --color always

FROM debian:buster-slim as application-base

RUN apt-get update -y
RUN apt-get upgrade -y

FROM application-base as application

ENV MODE="production" \
    PORT=8080 \
    RUST_LOG="info"

COPY --from=project /project/release/acme /acme

EXPOSE ${PORT}/tcp
EXPOSE ${PORT}/udp

ENTRYPOINT ["./acme"]