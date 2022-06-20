pub use logger::*;

pub mod logger;

pub enum Loggers {
    Debug,
    Info,
    Tracing,
}