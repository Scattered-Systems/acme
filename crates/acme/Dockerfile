FROM jo3mccain/rusty:latest as builder

ADD . /project
WORKDIR /project

COPY . .
RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /project/target/release/app /bin/app

ENTRYPOINT ["./app"]