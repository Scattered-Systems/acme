/*
    Appellation: acme-macros <library>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/

use std::net::SocketAddr;

#[macro_export]
macro_rules! netaddr {
    ( $( ($x:expr, $y:expr) ),* ) => {
        {
            let mut tmp = Vec::new();
            $(
                tmp.push(SocketAddr::from(($x, $y)));
            )*
            tmp
        }
    };
}
