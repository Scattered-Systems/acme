pub(crate) mod logger;

pub use crate::actors::loggers::logger::*;

pub enum Loggers {
    Debug,
    Info,
    Tracing,
}