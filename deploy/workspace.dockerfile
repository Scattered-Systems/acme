FROM ubuntu as builder-base

# Update default packages
RUN apt-get update

# Get Ubuntu packages
RUN apt-get install -y \
    build-essential \
    cmake \
    curl

# Update new packages
RUN apt-get update

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

FROM builder-base as builder

ENV MODE=false \
    NETWORK_MASTER_PORT=9999

ADD . /app
WORKDIR /app

COPY . .
RUN cargo build --workspace --release