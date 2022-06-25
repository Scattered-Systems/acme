use crate::{LocalTime, TimeStamp};

pub fn timestamp() -> TimeStamp {
    LocalTime::now().into()
}