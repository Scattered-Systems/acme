/*
    Appellation: acme <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        Acme is designed to simplify the creation of agile web applications written in Rust
*/
#[doc(inline)]
pub use self::{actors::*, contexts::*, core::*, data::*};


mod actors;
mod contexts;
mod core;
mod data;
