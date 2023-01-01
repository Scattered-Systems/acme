/*
    Appellation: xtask <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::utils::*;

pub(crate) mod utils;

pub mod cli;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("Welcome to xtask...");

    let handle = std::thread::spawn(move || {
        cli::handle().join().unwrap();
    });
    handle.join().ok().unwrap();

    Ok(())
}

pub fn cli() -> anyhow::Result<()> {
    let matches = clap::builder::Command::new("app");

    Ok(())
}

///
pub type Bundle<T = String> = std::collections::HashMap<T, Vec<Vec<T>>>;

///
#[macro_export]
macro_rules! cmd {
    ($(
        $x:expr;
        [ $( $y:expr ),* ]
    );*) => {
        {
            $(
                let mut cmd = std::process::Command::new($x);
                cmd.current_dir(scsys_xtask::project_root());
                let mut tmp = Vec::new();
                $(
                    tmp.push($y);
                )*
                cmd.args(tmp.as_slice()).status().expect("");
            )*
        }
    };
}