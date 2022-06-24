pub(crate) mod logger;

pub use super::loggers::*;

pub enum Loggers {
    Debug,
    Info,
    Tracing,
}