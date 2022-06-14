FROM jo3mccain/rusty as builder

ENV MODE=false \
    NETWORK_MASTER_PORT=9999

ADD . /app
WORKDIR /app

COPY . .
RUN cargo test --workspace
RUN cargo build --release