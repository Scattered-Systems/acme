use std::collections::HashMap;

use bson;

pub mod structs;

pub type Buffer = HashMap<String, String>;
pub type Clock = bson::DateTime;
pub type Bid = bson::oid::ObjectId;