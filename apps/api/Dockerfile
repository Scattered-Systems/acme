FROM jo3mccain/rusty:latest as builder

ADD . /bin
WORKDIR /bin

COPY . .
RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /bin/target/release/acme-api /bin/acme-api

ENV DEV_MODE=false \
    SERVER_PORT=9999

CMD ["./bin/acme-api"]