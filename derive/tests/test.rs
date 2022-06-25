#[cfg(test)]
mod basics {
    #[test]
    fn compiles() {
        let f = |x: f32, y: f32| x.powf(y);
        assert_eq!(f(2.0, 3.0), 8.0)
    }

    #[test]
    fn derive_sample_function() {
        use acme_derive::SampleFunction;

        #[derive(Clone, Debug, SampleFunction)]
        pub struct Demo;

        let res = sample();
        assert_eq!(res, 18u16)
    }
}


