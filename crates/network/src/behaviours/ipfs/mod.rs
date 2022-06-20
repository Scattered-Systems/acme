pub use subnet::Subnet;

mod mainnet;
// TODO - Implement a standard IPFS Node
mod subnet;


pub mod utils {
    use std::{env, error::Error, fs, path::Path, str::FromStr};

    use libp2p::{
        Multiaddr,
        multiaddr::Protocol,
    };

    fn get_ipfs_path() -> Box<Path> {
        env::var("IPFS_PATH")
            .map(|ipfs_path| Path::new(&ipfs_path).into())
            .unwrap_or_else(|_| {
                env::var("HOME")
                    .map(|home| Path::new(&home).join(".ipfs"))
                    .expect("could not determine home directory")
                    .into()
            })
    }

    /// Read the pre shared key file from the given ipfs directory
    fn get_psk(path: Box<Path>) -> std::io::Result<Option<String>> {
        let swarm_key_file = path.join("swarm.key");
        match fs::read_to_string(swarm_key_file) {
            Ok(text) => Ok(Some(text)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// for a multiaddr that ends with a peer id, this strips this suffix. Rust-libp2p
    /// only supports dialing to an address without providing the peer id.
    fn strip_peer_id(addr: &mut Multiaddr) {
        let last = addr.pop();
        match last {
            Some(Protocol::P2p(peer_id)) => {
                let mut addr = Multiaddr::empty();
                addr.push(Protocol::P2p(peer_id));
                println!(
                    "removing peer id {} so this address can be dialed by rust-libp2p",
                    addr
                );
            }
            Some(other) => addr.push(other),
            _ => {}
        }
    }

    /// parse a legacy multiaddr (replace ipfs with p2p), and strip the peer id
    /// so it can be dialed by rust-libp2p
    fn parse_legacy_multiaddr(text: &str) -> Result<Multiaddr, Box<dyn Error>> {
        let sanitized = text
            .split('/')
            .map(|part| if part == "ipfs" { "p2p" } else { part })
            .collect::<Vec<_>>()
            .join("/");
        let mut res = Multiaddr::from_str(&sanitized)?;
        strip_peer_id(&mut res);
        Ok(res)
    }
}