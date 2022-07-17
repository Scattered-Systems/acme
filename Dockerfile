FROM photon as container

RUN yum install -y \
    apt-utils \
    build-essential  \
    cmake \
    pkg-config

FROM container as environment

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

FROM environment as project

ADD . /project
WORKDIR /project

COPY . .
RUN cargo fmt --all && \
    cargo build --workspace --release --verbose --color always && \
    cargo test --all-features --verbose --color always

FROM photon as app

ENV MODE="production" \
    PORT=8080 \
    RUST_LOG="info"

COPY --from=project /project/release/acme /acme

EXPOSE ${PORT}/tcp
EXPOSE ${PORT}/udp

ENTRYPOINT ["./acme"]