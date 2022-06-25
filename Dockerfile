FROM jo3mccain/rusty as builder

ADD . /project
WORKDIR /project

COPY . .
RUN cargo test --all-features --workspace && \
    cargo build --release

FROM debian:buster-slim as application

ENV DEV_MODE=false

COPY --from=builder /project/target/release/acme-cli /acme-cli

ENTRYPOINT ["./acme-cli"]