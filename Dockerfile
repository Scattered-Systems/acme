FROM rust as builder-base

RUN rustup component add rustfmt

FROM builder-base as builder

ADD . /bin
WORKDIR /bin

COPY . .
RUN cargo fmt --all && \
    cargo build --workspace --release && \
    cargo test --all-features

FROM photon as app

ENV MODE="production" \
    PORT=8080 \
    RUST_LOG="info"

COPY --from=builder /bin/target/release/acme /acme

EXPOSE ${PORT}/tcp
EXPOSE ${PORT}/udp

ENTRYPOINT ["./acme"]