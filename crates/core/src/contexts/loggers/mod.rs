pub use logger::*;

mod logger;

pub enum LoggerStates {
    Debug,
    Info,
    Tracing,
}
