use bson;

pub type Clock = bson::DateTime;
pub type Container<T> = std::collections::HashMap<T, Vec<T>>;
pub type ContentId = String;
pub type ObjectId = bson::oid::ObjectId;
pub type Transaction = Container<String>;

pub mod keys {
    use libp2p;

    pub type KeyPair = libp2p::identity::Keypair;
}

pub mod stateful {
    pub enum Connections {
        Connecting,
        Connected,
    }

    pub enum Crud {
        Creating,
        Reading,
        Updating,
        Delete,
    }

    pub enum Chains {
        Creating,
        Computing,
    }
}