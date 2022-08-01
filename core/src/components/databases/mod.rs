/*
    Appellation: databases <module>
    Description:
        ... Summary ...
*/
pub use database::*;
pub use utils::*;

mod database;

mod utils {
    pub fn create_db_uri(
        prefix: String,
        username: String,
        password: String,
        host: String,
        port: u16,
        suffix: String,
    ) -> String {
        format!(
            "{}://{}:{}@{}:{}/{}",
            prefix, username, password, host, port, suffix
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uri_formatter() {
        let actual = create_db_uri(
            "postgres".to_string(),
            "postgres".to_string(),
            "example".to_string(),
            "localhost".to_string(),
            5432,
            "postgres".to_string(),
        );
        let expected: String = "postgres://postgres:example@localhost:5432/postgres".to_string();
        assert_eq!(&actual, &expected)
    }
}
