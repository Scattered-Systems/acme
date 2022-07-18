/*
   Appellation: primitives
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use common::*;
pub use utils::*;

mod common {
    pub use constants::*;
    pub use controllers::*;
    pub use types::*;

    mod constants {
        pub const COINBASE_API_ENDPOINT: &str = "https://coinbase.com/api/v2";
        pub const COINBASE_PRO_ENDPOINT: &str = "https://pro.coinbase.com/api";
        pub const STANDARD_OAUTH_TOKEN_PATH: &str = "oauth/token";
    }

    mod controllers {
        /// Outlines the required functionality of an Exchangeable item
        pub trait Exchangeable<Act, Conf, Cont, Data> {
            fn actor(&self, context: Cont) -> Act
                where
                    Self: Sized;
            fn configure(&self, config: Conf) -> Result<Self, config::ConfigError>
                where
                    Self: Sized;
            fn create(&self, data: Vec<Data>) -> Self
                where
                    Self: Sized;
            fn describe(&self) -> String
                where
                    Self: Sized;
        }

        /// Outlines the required functionality of a malleable item
        /// def. Malleable is defined as the characteristic of being allowed to make alterations
        pub trait Malleable<Actor, Conf, Cont, Data> {
            fn action(&self, actor: Actor) -> Self
                where
                    Self: Sized;
            fn configure(&self) -> Result<Self, config::ConfigError>
                where
                    Self: Sized;
            fn create(&self, actor: Actor, config: Conf) -> Self
                where
                    Self: Sized;
        }

        #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
        pub enum Shapes {
            Hexagon,
            Octagon,
            Pentagon,
            Square,
            Triangle,
        }

        #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
        pub struct Transaction<T = String> {
            pub id: u64,
            pub hash: String,
            pub key: String,
            pub timestamp: i64,
            pub data: Vec<T>,
        }

        impl<T> Transaction<T> {
            fn create(id: u64, hash: String, key: String, timestamp: i64, data: Vec<T>) -> Self {
                Self {
                    id,
                    hash,
                    key,
                    timestamp,
                    data,
                }
            }
            pub fn new(id: u64, hash: String, key: String, timestamp: i64, data: Vec<T>) -> Self {
                Self::create(id, hash, key, timestamp, data)
            }
        }

        #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
        pub struct Container<T = String> {
            pub id: u64,
            pub hash: String,
            pub key: String,
            pub nonce: u64,
            pub secret: String,
            pub timestamp: i64,
            pub transactions: Vec<T>,
        }

        impl<T> Container<T> {
            fn create(
                id: u64,
                hash: String,
                key: String,
                nonce: u64,
                secret: String,
                timestamp: i64,
                transactions: Vec<T>,
            ) -> Self {
                Self {
                    id,
                    hash,
                    key,
                    nonce,
                    secret,
                    timestamp,
                    transactions,
                }
            }
            pub fn new(
                id: u64,
                hash: String,
                key: String,
                nonce: u64,
                secret: String,
                timestamp: i64,
                transactions: Vec<T>,
            ) -> Self {
                Self::create(id, hash, key, nonce, secret, timestamp, transactions)
            }
        }

        #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
        pub enum Alias {
            Personal,
            Social,
            Work,
        }

        #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
        pub enum Abbr<T> {
            Full(T),
            Short(T),
        }

        #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
        pub enum Addresses {
            Street {
                primary: String,
                secondary: String,
                city: String,
                state: String,

                zip_code: String,
            },
        }

        #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
        pub enum Names {
            FullName {
                prefix: String,
                first: String,
                middle: String,
                last: String,
                suffix: String,
            },
            NameOnly {
                first: String,
                middle: String,
                last: String,
            },
        }

        #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
        pub struct Appellation {
            pub alias: String,
            pub name: String,
        }

        #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
        pub struct Descriptor {
            pub id: u64,
            pub hash: String,
            pub key: String,
            pub title: String,
            pub audience: String,
            pub content: String,
            pub data: Vec<String>,
        }

        /// Destined to control the named objects with characteristics found in Descriptor
        #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
        pub enum Descriptors {
            Basic(Descriptor),
        }
    }

    mod types {
        /// Describes a boxed dynamic error with Send, Sync and 'static tags enabled
        pub type AsyncError = Box<dyn std::error::Error + Send + Sync + 'static>;
        /// Describes a boxed dynamic error
        pub type StandardError = Box<dyn std::error::Error>;
        /// Describes a configuration builder in their default state
        pub type DefaultConfigBuilder = config::ConfigBuilder<config::builder::DefaultState>;
        /// Describes the result of a collection of configuration files
        pub type ConfigFileCollection =
        Vec<config::File<config::FileSourceFile, config::FileFormat>>;
    }
}

mod utils {
    /// Creates a collection of configuration files found within the working directory and following the provided pattern
    pub fn collect_config_files(pattern: &str, required: bool) -> crate::ConfigFileCollection {
        let f = |pat: &str, opt: bool| {
            glob::glob(pat)
                .unwrap()
                .map(|path| config::File::from(path.unwrap()).required(opt))
                .collect::<Vec<_>>()
        };
        f(pattern, required)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize| x.pow(x.try_into().unwrap());
        assert_eq!(f(2), 4)
    }
}
