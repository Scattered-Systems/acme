/*
    Appellation: mod
    Context:
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
 */
pub use crate::configurations::{standard::*, utils::*};

mod standard;

mod utils {
    use config::{ConfigBuilder, builder::DefaultState};

    pub type ConfigBuilderDS = ConfigBuilder<DefaultState>;

    pub fn config_std_constructor(pattern: String) -> ConfigBuilderDS {
        let mut builder = config::Config::builder();

        collect_config_files(builder, &pattern)
    }

    pub fn collect_config_files(builder: ConfigBuilderDS, pattern: &str) -> ConfigBuilderDS {
        builder.add_source(
            glob::glob(pattern)
                .unwrap()
                .map(|path| config::File::from(path.unwrap()).required(false))
                .collect::<Vec<_>>()
        )
    }
}