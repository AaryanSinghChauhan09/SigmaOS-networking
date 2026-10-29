//! Network address types for SigmaOS networking layer.
use std::fmt;
use std::str::FromStr;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

pub use std::net::{IpAddr as IpAddress, SocketAddr as SocketAddress};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Endpoint {
    pub host: String,
    pub port: u16,
}

impl Endpoint {
    pub fn new(host: impl Into<String>, port: u16) -> Self {
        Self { host: host.into(), port }
    }
    pub fn loopback(port: u16) -> Self {
        Self::new("127.0.0.1", port)
    }
    pub fn any(port: u16) -> Self {
        Self::new("0.0.0.0", port)
    }
}

impl fmt::Display for Endpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.host, self.port)
    }
}

impl FromStr for Endpoint {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.rsplitn(2, ':');
        let port_str = parts.next().ok_or_else(|| format!("missing port in {s}"))?;
        let host = parts.next().ok_or_else(|| format!("missing host in {s}"))?.to_string();
        let port = port_str.parse::<u16>().map_err(|e| format!("bad port: {e}"))?;
        Ok(Endpoint::new(host, port))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endpoint_display() {
        // Arrange
        let ep = Endpoint::new("192.168.1.1", 8080);
        // Assert
        assert_eq!(ep.to_string(), "192.168.1.1:8080");
    }

    #[test]
    fn test_endpoint_parse() {
        // Arrange / Act
        let ep: Endpoint = "10.0.0.1:443".parse().unwrap();
        // Assert
        assert_eq!(ep.host, "10.0.0.1");
        assert_eq!(ep.port, 443);
    }

    #[test]
    fn test_loopback_convenience() {
        // Arrange / Act
        let ep = Endpoint::loopback(3000);
        // Assert
        assert_eq!(ep.host, "127.0.0.1");
        assert_eq!(ep.port, 3000);
    }

    #[test]
    fn test_parse_invalid_returns_error() {
        // Arrange / Act
        let result = "no-port".parse::<Endpoint>();
        // Assert
        assert!(result.is_err());
    }
}
