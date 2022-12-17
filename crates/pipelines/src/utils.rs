/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/

/// Remove the first and last charecters of a string
pub fn fnl_remove<T: Clone + ToString>(data: T) -> String {
    let data = data.to_string();
    let mut chars = data.chars();
    chars.next();
    chars.next_back();
    chars.as_str().to_string()
}

pub fn remove_dir_all() {}
