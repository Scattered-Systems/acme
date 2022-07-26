# acme

[![Code Analysis](https://github.com/FL03/acme/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/FL03/acme/actions/workflows/rust-clippy.yml)
[![Rust](https://github.com/FL03/acme/actions/workflows/rust.yml/badge.svg)](https://github.com/FL03/acme/actions/workflows/rust.yml)

Acme was created to be a collection of useful clients, interfaces, and mission-critical application scaffolding for
quickly designing enterprise grad dApps for the Scattered-Systems Ecosystem.

## Developers

### Getting Started

    git clone https://github.com/scattered-systems/acme.git

#### Containers

    docker build . --tag jo3mccain/acme:next
    docker run jo3mccain/acme:next

_with compose_

    docker -f ".docker/docker-compose.yml" --name acme up
    