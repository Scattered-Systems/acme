FROM jo3mccain/rusty:latest as builder

ADD . /project
WORKDIR /project

COPY . .
RUN cargo build --release --package app

FROM debian:buster-slim

COPY --from=builder /project/target/release/app /project/app

ENV DEV_MODE=false \
    PORT=9999

EXPOSE ${PORT}
CMD ["./project/app"]