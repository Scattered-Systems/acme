FROM jo3mccain/rusty as builder

ENV CRATE_NAME=acme-cli

ADD . /project
WORKDIR /project

COPY . .
RUN cargo build --release --package ${CRATE_NAME}

FROM debian:buster-slim as application

ENV CRATE_NAME=acme-cli \
    DEV_MODE=false

COPY --from=builder /project/target/release/${CRATE_NAME} /${CRATE_NAME}

ENTRYPOINT ["./${CRATE_NAME}"]