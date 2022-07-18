FROM rust as builder-base

RUN rustup component add rustfmt

FROM builder-base as builder

ADD . /bin
WORKDIR /bin

COPY . .
RUN cargo fmt --all && \
    cargo build --workspace --release && \
    cargo test --all-features --release