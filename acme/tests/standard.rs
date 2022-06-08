use acme::utils::timestamp;

#[cfg(test)]

#[test]
fn basic() {
    println!("{:#?}", timestamp());
    let f = |x: f32, y: f32| x.powf(y);
    assert_eq!(f(2.0, 3.0), 8.0)
}