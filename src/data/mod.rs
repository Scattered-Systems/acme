use bson;

pub mod structs;

pub type Clock = bson::DateTime;
pub type ObjectId = bson::oid::ObjectId;

