/*
    Appellation: types <primitives>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/

/// type alias for [axum::Json]
pub type AxumJson<T = JsonVal> = axum::Json<T>;
/// type alias for [axum::Router]
pub type AxumRouter = axum::Router;

/// Type alias for [serde_json::Value]
pub type JsonVal = serde_json::Value;