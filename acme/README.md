# acme

Acme enables developers to design data-centric, cloud-native applications within Rust wrapping popular frameworks
such as Axum, Clap, and Tokio into a single import.

## Getting Started

### Building from the source

    git clone https://github.com/scattered-systems/acme
    cargo build && cargo build --release
    cargo test --all-features --color always --release --verbose --workspace

### Examples

    use acme;
    
    fn main() {
        println!("Welcome to Acme");
        
    }