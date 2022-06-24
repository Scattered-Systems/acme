/*
    Appellation: Test
    Context: External module
    Creator: FL03 <jo3mccain@icloud.com> (https://pzzld.eth.link/)
    Description:
 */

extern crate acme_derive;

#[cfg(test)]
mod tests {

    #[test]
    fn basic() {
        let f = |x: f32, y: f32| x.powf(y);
        assert_eq!(f(2.0, 3.0), 8.0)
    }

    #[test]
    fn test_sample_derive() {
        use acme_derive::SampleFunction;

        #[derive(SampleFunction)]
        struct Tmp;

        let res: u16 = sample();
        assert_eq!(18, res.clone())
    }
}
