FROM jo3mccain/rusty:latest as builder

ADD . /project
WORKDIR /project

COPY . .
RUN cargo build --release --package acme-cli

FROM debian:buster-slim

COPY --from=builder /project/target/release/acme-cli /project/acme-cli

ENV DEV_MODE=false \
    PORT=9999

EXPOSE ${PORT}
CMD ["./project/acme-cli"]