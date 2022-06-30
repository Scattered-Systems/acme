FROM jo3mccain/rusty as builder

ENV CARGO_TERM_COLOR=always

ADD . /project
WORKDIR /project

COPY . .
RUN cargo build --release --workspace --quiet --color ${CARGO_TERM_COLOR}

FROM debian:buster-slim as application

ENV CRATE_NAME=acme-node \
    DEV_MODE=false

COPY --from=builder /project/target/release/$CRATE_NAME /$CRATE_NAME

ENTRYPOINT ["./$CRATE_NAME"]