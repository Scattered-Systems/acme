/*
    Appellation: acme <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        Acme was built to facilitate the creation of secure peer-to-peer applications written natively in WebAssembley
        
*/
#[doc(inline)]
pub use self::core::*;

pub(crate) mod core;
#[cfg(feature = "clusters")]
pub use acme_clusters as clusters;
#[cfg(feature = "conduits")]
pub use acme_conduits as conduits;
#[cfg(feature = "minis")]
pub use acme_minis as minis;

pub mod prelude {}
