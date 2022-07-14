#!/usr/bin/env bash
cargo build --release --workspace
cargo fmt --all
cargo test --release --all-features