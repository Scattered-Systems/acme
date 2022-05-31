use crate::primitives;


pub struct Cache {
    pub id: primitives::ContentId,
    pub timestamp: primitives::types::Clock,
    pub data: primitives::types::Container<String>
}
