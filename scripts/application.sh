#!/usr/bin/env bash
cargo build --release --package acme --bin acme-api
cargo run --release --package acme --bin acme-api