/*
    Appellation: acme-derive
    Context: Library
    Creator: FL03 <jo3mccain@icloud.com> (pzzld.eth)
    Description:
        This library is dedicated towards creating derive macros for the acme in support of the
        Scattered-Systems ecosystem.

        Goals:
            *
 */
extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_derive(SampleFunction)]
pub fn derive_sample_function(_item: TokenStream) -> TokenStream {
    "fn sample() -> u16 { 18 }".parse().unwrap()
}