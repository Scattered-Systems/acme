pub use crate::actors::loggers::logger::*;

pub(crate) mod logger;

pub enum Loggers {
    Debug,
    Info,
    Tracing,
}