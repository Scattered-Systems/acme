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
                tmp.push(std::net::SocketAddr::from(($x, $y)));
            )*
            tmp
        }
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_netaddr() {
        let a = netaddr![([0, 0, 0, 0], 8080)];
        let b = vec![std::net::SocketAddr::from([0, 0, 0, 0], 8080)];
        assert_eq!(a, b)
    }
}
