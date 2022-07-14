/*
   Appellation: interface
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use clap::Parser;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Interface {
    pub development: bool,
    pub name: String,
}

impl Interface {
    fn configure<T: Clone + std::str::FromStr>(
        &self,
        data: Vec<T>,
    ) -> Result<Self, config::ConfigError> {
        let mut builder = config::Config::builder();
        builder = builder
            .set_default("development", true)?
            .set_default("name", "acme")?;
        builder.build()?.try_deserialize()
    }
    fn create(development: bool, name: String) -> Self {
        Self { development, name }
    }
    pub fn new(development: bool, name: String) -> Self {
        Self::create(development, name)
    }
    pub fn run(&self) -> Result<crate::CommandCenter, acme::StandardError> {
        let args = crate::CommandCenter::parse();
        println!("{:#?}", &args);

        Ok(args.clone())
    }
}
