#[cfg(test)]
mod basics {
    use acme_derive::*;

    #[test]
    fn derive_sample_function() {
        use acme_derive::SampleFunction;

        #[derive(Clone, Debug, SampleFunction)]
        pub struct Demo;

        let res = sample();
        assert_eq!(res, 18u16)
    }

    #[test]
    fn test_derive_description() {
        #[derive(Describe)]
        struct Apps {
            name: String,
            slug: String,
        }

        assert_eq!(Apps::describe(), "Apps is a struct with these named fields: name, slug.")
    }
}


