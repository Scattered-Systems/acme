use chrono;
use crate::primitives::Clock;

pub fn timestamp() -> Clock {
    let current_time: Clock = chrono::Local::now().into();
    return current_time
}