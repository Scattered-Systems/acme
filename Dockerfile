FROM jo3mccain/rusty as builder

ADD . /project
WORKDIR /project

COPY . .
RUN cargo test -p acme --all-features && \
    cargo build --release --workspace

FROM debian:buster-slim as application

ENV DEV_MODE=false \
    PORT=8080

COPY --from=builder /project/target/release/acme-cli /acme-cli

EXPOSE ${PORT}/tcp
EXPOSE ${PORT}/udp


ENTRYPOINT ["./acme-cli"]