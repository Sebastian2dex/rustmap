/*
    This file deals with DNS resolution and handling IP
    IpAddr: Enum that can only hold either Ipv4 or Ipv6
    Ipv4Addr: Filter out Ipv4 from IpAddr
    ToSocketAddrs: Needed for DNS resolution
 */

use std::net::{IpAddr, ToSocketAddrs, Ipv4Addr};

// No ownership needed, just borrow the input
pub fn resolve_target(input: &str) -> Result<Ipv4Addr, String> {
    if input.eq_ignore_ascii_case("localhost") {
        return Ok(Ipv4Addr::new(127, 0, 0, 1));
    }
    
    // Parse IP address
    if let Ok(ip) = input.parse::<IpAddr>() {
        match ip {
            IpAddr::V4(ipv4) => return Ok(ipv4),
            IpAddr::V6(_) => {
                return Err(format!("Not performing on Ipv6: {}", input));
            }
        }
    }

    // DNS resolution
    // Creates a string like "example.com:0" as toSocketAddrs require host:port format
    // We will just do IP grabbing just by using port 0 as a dummy
    // We are not concerned to actually handle ports
    // And, we are only concerned with IPv4
    let addresses = format!("{}:0", input)
        .to_socket_addrs()
        .map_err(|e| format!("Failed to resolve '{}': {}", input, e))?;

    // filter for Ipv4
    for addr in addresses {
        // SocketAddr contains both IP and Port
        // addr.ip() returns only the IP address part and we filter out IPv4 only
        if let IpAddr::V4(ipv4) = addr.ip() {
            return Ok(ipv4);
        }
    }
    // Return the failure message if no Ipv4 found
    Err(format!("No Ipv4 found for {}", input))
}