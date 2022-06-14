pub trait Node {
    type Configuration;

    fn new() -> Self;
    fn setup(&self) -> Self;
}