FROM ubuntu as builder-base

RUN apt-get install -y  \
    build-essential  \
    curl

RUN apt-get update

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

FROM builder-base as builder

RUN apt-get install -y cmake && \
    rustup install toolchain nightly && \
    rustup target add wasm32-unknown-unknown --toolchain nightly

ADD . /app
WORKDIR /app

COPY . .
RUN cargo build --release

FROM debian:buster-slim as application

COPY --from=builder /app/target/release/app /app

ENV DEV_MODE=false \
    PORT=9999

EXPOSE ${PORT}
ENTRYPOINT ["./app"]