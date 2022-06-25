use super::NodeSpec;

#[derive(Clone, Debug)]
pub struct Node<T = std::collections::HashMap<String, Vec<String>>> {
    pub appellation: String,
    pub context: T
}

impl NodeSpec for Node {
    type Appellation = String;
    type Client = ();
    type Configuration = ();
    type Data = ();

    fn activate(appellation: Self::Appellation) -> Self { todo!() }

    fn configure(configuration: Self::Configuration) -> Self {
        todo!()
    }

    fn connect(&mut self) -> Self::Client {
        todo!()
    }

    fn describe(&mut self) -> Self::Data {
        todo!()
    }
}