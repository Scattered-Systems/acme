/*
    Appellation: loggers <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Logger {
    pub level: String,
}

impl Logger {
    fn constructor(level: String) -> Self {
        Self { level }
    }
    pub fn new(level: String) -> Self {
        Self::constructor(level)
    }
    pub fn from<T: Clone + std::string::ToString>(level: T) -> Self {
        Self::constructor(level.to_string())
    }
    pub fn setup(self) {
        if std::env::var_os("RUST_LOG").is_none() {
            std::env::set_var("RUST_LOG", self.level.clone());
        }
        tracing_subscriber::fmt::init();
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::from("info")
    }
}

impl std::fmt::Display for Logger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Logger(level={})", self.level)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger() {
        let actual = Logger::from("info");
        let expected = Logger::default();
        assert_eq!(&actual, &expected)
    }
}
