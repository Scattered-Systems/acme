/*
    Appellation: s3 <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/

pub mod buckets;

pub fn buckets() -> axum::Router {
    buckets::router()
}
