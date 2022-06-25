FROM jo3mccain/rusty as builder

ADD . /project
WORKDIR /project

COPY . .
RUN cargo test --all-features && \
    cargo package --all-features --allow-dirty && \
    cargo build --release

FROM debian:buster-slim as application

ENV DEV_MODE=false \
    PORT=8080

COPY --from=builder /project/target/release/acme-cli /acme-cli

EXPOSE 7545/tcp
EXPOSE 8545/tcp
EXPOSE 9090/tcp
EXPOSE 9090/udp

ENTRYPOINT ["./acme-cli"]