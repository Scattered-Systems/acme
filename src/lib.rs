/*
   Appellation: acme
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use crate::{common::*, utils::*};

#[doc(inline)]
#[cfg(feature = "default")]
pub use acme_sdk::*;
#[doc(inline)]
#[cfg(feature = "full")]
pub use acme_sdk::*;

mod common {
    pub use constants::*;
    pub use controllers::*;
    pub use types::*;

    mod constants {}

    mod controllers {
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

        pub trait Shape<Actor, Conf, Cont, Data> {
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
        pub type BoxedError = Box<dyn std::error::Error>;
    }
}

mod utils {
    pub fn collect_config_files(pattern: &str, required: bool) -> acme_sdk::ConfigFromFileVec {
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
