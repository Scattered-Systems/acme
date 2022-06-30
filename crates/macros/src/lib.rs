/*
   Appellation: acme-macros
   Context: Library
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       This crate is dedicated to acme for designing highly-optimized blockchain's in support of
       the Scattered-Systems ecosystem
*/
extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn simple_attr(attr: TokenStream, input: TokenStream) -> TokenStream {
    "".parse().unwrap()
}
