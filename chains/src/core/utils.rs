/*
   Appellation: utils
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use dates::*;

mod dates {
    use crate::BTStamp;

    #[derive(Clone, Debug, Hash, serde::Deserialize, PartialEq, serde::Serialize)]
    pub struct BlockStamp;

    impl BlockStamp {
        pub fn local() -> BTStamp { block_ts_local() }
        pub fn utc() -> BTStamp {
            block_ts_utc()
        }
    }

    pub fn block_ts_local() -> BTStamp { chrono::Local::now().timestamp() }

    pub fn block_ts_utc() -> BTStamp {
        chrono::Utc::now().timestamp()
    }
}
