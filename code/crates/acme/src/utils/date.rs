use crate::primitives::types::{LocalTime, TimeStamp};


pub fn timestamp() -> Stamp {
    let current_time: TimeStamp = LocalTime::now().into();
    return current_time
}