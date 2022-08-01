/*
    Appellation: verifier <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Verifier<T> {
    pub data: Vec<T>,
}

impl<T> Verifier<T> {
    fn constructor(data: Vec<T>) -> Self {
        Self { data }
    }
    pub fn new(data: Vec<T>) -> Self {
        Self::constructor(data)
    }
}

impl<T> Default for Verifier<T> {
    fn default() -> Self {
        Self::constructor(Vec::<T>::new())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize, y: usize| x + y;
        assert_eq!(f(4, 2), 6)
    }
}
