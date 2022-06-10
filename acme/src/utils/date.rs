use crate::primitives::date::{Local, Stamp};


pub fn timestamp() -> Stamp {
    let current_time: Stamp = Local::now().into();
    return current_time
}