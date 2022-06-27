#[cfg(test)]
mod basics {
    #[test]
    fn compiles() {
        let f = |x: f32, y: f32| x.powf(y);
        assert_eq!(f(2.0, 3.0), 8.0)
    }

    #[test]
    fn test_container() {
        let now = acme_data::Localtime::now();
        assert_eq!(now, now)
    }
}
