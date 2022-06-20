# TODO - Deconstruct the environment into seperate build-stages for external targetting
FROM jo3mccain/rusty as builder-base

ADD . /project
WORKDIR /project

COPY . .
RUN cargo test --workspace

FROM builder-base as builder

RUN cargo build --release -p app && \
    cargo package -p acme --features full && \
    cargo publish --

FROM debian:buster-slim as application

ENV DEV_MODE = false \
    PORT = 9090

COPY --from=builder /bin/target/release/app /bin/app

EXPOSE ${SERVER_PORT}
ENTRYPOINT ["./app"]