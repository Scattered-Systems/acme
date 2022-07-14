#!/usr/bin/env bash
cargo build --release --package acme --bin acme-cli
cargo run --release --package acme --bin acme-cli