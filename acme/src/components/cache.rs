use crate::primitives::{Clock, Container, ObjectId};


pub struct Cache {
    pub id: ObjectId,
    pub key: String,
    pub timestamp: Clock,
    pub data: Container<String>
}
