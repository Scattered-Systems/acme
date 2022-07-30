/*
    Appellation: database <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum DbUri {
    Reg(String),
    Std(TemplateUri),
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct TemplateUri {
    pub prefix: String,
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub suffix: String,
}

impl TemplateUri {
    pub fn new(
        prefix: String,
        username: String,
        password: String,
        host: String,
        port: u16,
        suffix: String,
    ) -> Self {
        Self {
            prefix,
            username,
            password,
            host,
            port,
            suffix,
        }
    }
    pub fn into_string(self) -> String {
        create_db_uri(
            self.prefix,
            self.username,
            self.password,
            self.host,
            self.port,
            self.suffix,
        )
    }
}

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

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DatabaseConnection {
    pub name: String,
    pub uri: String,
}

impl DatabaseConnection {
    pub fn new(name: String, uri: DbUri) -> Self {
        let uri = match uri {
            DbUri::Reg(v) => v.clone(),
            DbUri::Std(v) => v.into_string().clone(),
        };
        Self { name, uri }
    }
}

pub struct Database {
    pub appellation: (String, String, String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let uri = TemplateUri::new(
            "postgres".to_string(),
            "postgres".to_string(),
            "example".to_string(),
            "localhost".to_string(),
            5432,
            "postgres".to_string(),
        );
        let formatted: String = uri.clone().into_string();
        assert_eq!(&formatted, &formatted)
    }
}
