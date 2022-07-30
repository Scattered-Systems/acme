/*
    Appellation: data <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use utils::*;

pub mod models;

mod utils {
    pub fn check_path(path: String) -> bool {
        let fp: &std::path::Path = std::path::Path::new(&path);
        fp.exists()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_path() {
        let actual: bool = check_path("".to_string());
        let expected: bool = false;
        assert_eq!(&actual, &expected)
    }
}
