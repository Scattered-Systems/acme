# Quickstart

Welcome to acme

## Getting Started

### _Building from the Source_

#### Clone the repository

    git clone https://gitlab.com/FL03/acme

#### Crate

    cargo build --color always --release --workspace
    cargo test --all-features --color always --release --verbose --workspace

## Examples

    use acme;

    #[tokio]
    async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

        Ok(())
    }