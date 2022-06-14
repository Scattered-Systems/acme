#[cfg(test)]
#[test]
fn basic() {
    let f = |x: f32, y: f32| x.powf(y);
    assert_eq!(f(2.0, 3.0), 8.0)
}

#[test]
fn test_timestamp() {
    use acme::types::{LocalTime, TimeStamp};
    let timestamp: TimeStamp = LocalTime::now().into();
    assert_eq!(&timestamp, &timestamp)
}