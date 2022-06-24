/*
    Appellation: Test
    Context: External module
    Creator: FL03 <jo3mccain@icloud.com> (https://pzzld.eth.link/)
    Description:
 */

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {
        let f = |x: f32, y: f32| x.powf(y);
        assert_eq!(f(2.0, 3.0), 8.0)
    }
}
