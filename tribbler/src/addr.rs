//! this module provides functions for getting interacting with the local
//! system's set of network interfaces.
use crate::err::TribResult;
use std::net::{IpAddr, ToSocketAddrs};

/// returns a list of the IP addresses on the current system's network
/// interfaces
pub fn get_local_addrs() -> TribResult<Vec<IpAddr>> {
    let r = local_ip_address::list_afinet_netifas()?;
    Ok(r.iter().map(|x| x.1).collect())
}

/// checks if the address provided in `addr` resolves to an IP address which is
/// currently served by one of the operating system's network interfaces.
pub fn check(addr: &str) -> TribResult<bool> {
    let addrs = addr.to_socket_addrs()?;
    let local_addrs = get_local_addrs()?;
    Ok(local_addrs
        .iter()
        .position(|&x| {
            addrs
                .clone()
                .map(|y| y.to_string().starts_with(&x.to_string()))
                .all(|i| i)
        })
        .map_or(false, |_| true))
}

/// module used to generate random ports to use in network addresses
pub mod rand {
    use rand::Rng;

    pub const PORT_START: u16 = 30001;
    pub const PORT_END: u16 = 35000;

    /// Generates a value for a port in the range of [[PORT_START], [PORT_END])
    ///
    /// ```rust
    /// use tribbler::addr::rand::rand_port;
    /// println!("port: {}", rand_port());
    /// ```
    pub fn rand_port() -> u16 {
        rand::thread_rng().gen_range(PORT_START..PORT_END)
    }

    /// Resolves an address ending with `:rand` to an actual port
    ///
    /// ```rust
    /// use tribbler::addr::rand::resolve;
    /// println!("{}", resolve("localhost:rand"))
    /// // localhost:3043
    /// ```
    pub fn resolve(s: &str) -> String {
        if s.ends_with(":rand") {
            format!("{}:{}", s.trim_end_matches(":rand"), rand_port())
        } else {
            s.to_string()
        }
    }

    /// shorthand for `resolve("localhost:rand")`
    pub fn local() -> String {
        resolve("localhost:rand")
    }
}
