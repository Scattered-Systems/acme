/*
   Appellation: validator
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Validator<Data> {
    pub source: String,
    pub data: Vec<Data>,
}

impl<Data> Validator<Data> {
    fn create(source: String, data: Vec<Data>) -> Self {
        Self { source, data }
    }
}

impl<Data> std::fmt::Display for Validator<Data> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Validator(source={})", self.source)
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
