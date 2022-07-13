FROM jo3mccain/rusty as builder

RUN rustup default stable && rustup update

ADD . /app
WORKDIR /app

COPY . .
RUN cargo build --workspace --release --verbose --color always

FROM debian:buster-slim as base

ENV DEV_MODE=false \
    CLUSTER_PORT=9090 \
    ETHEREUM_PORT=8545 \
    SERVER_PORT=8080

COPY --from=builder /app/target/release/acme-cli /acme-cli
COPY --from=builder /app/target/release/acme-api /acme-api

EXPOSE $CLUSTER_PORT/udp
EXPOSE $ETHEREUM_PORT/tcp
EXPOSE $SERVER_PORT/tcp
EXPOSE $SERVER_PORT/udp

ENTRYPOINT ["./acme-cli"]
CMD ["./acme-api"]