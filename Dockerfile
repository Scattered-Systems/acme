FROM jo3mccain/rusty as builder-base

ADD . /project
WORKDIR /project

COPY . .
RUN cargo test --workspace

FROM builder-base as builder

RUN cargo build --release -p acme

FROM debian:buster-slim as application

ENV DEV_MODE=false \
    PORT=8080

COPY --from=builder /bin/target/release/acme-bin /bin/acme-bin

EXPOSE ${PORT}/tcp
EXPOSE ${PORT}/udp

ENTRYPOINT ["./acme-bin"]