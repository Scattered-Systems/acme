FROM jo3mccain/rusty:latest as builder

ADD . /bin
WORKDIR /bin

COPY . .
RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /bin/target/release/acme-api /bin/acme-api

ENV DEV_MODE=false \
    PORT=8888

EXPOSE ${PORT}
CMD ["./bin/acme-api"]