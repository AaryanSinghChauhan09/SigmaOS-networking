//! DNS resolution abstractions for SigmaOS networking.
use std::collections::HashMap;
use std::net::IpAddr;

#[derive(Debug)]
pub enum DnsError {
    NotFound(String),
    Timeout,
    InvalidName(String),
}

impl std::fmt::Display for DnsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DnsError::NotFound(n) => write!(f, "DNS: no record for {n}"),
            DnsError::Timeout => write!(f, "DNS: query timed out"),
            DnsError::InvalidName(n) => write!(f, "DNS: invalid name {n}"),
        }
    }
}

/// DNS resolver trait — Inversion of Control, Interface Segregation.
pub trait Resolver {
    fn resolve(&self, hostname: &str) -> Result<Vec<IpAddr>, DnsError>;
}

/// Static/mocked resolver for testing.
#[derive(Default)]
pub struct StaticResolver {
    entries: HashMap<String, Vec<IpAddr>>,
}

impl StaticResolver {
    pub fn new() -> Self { Self::default() }
    pub fn add(&mut self, hostname: &str, addrs: Vec<IpAddr>) {
        self.entries.insert(hostname.to_lowercase(), addrs);
    }
}

impl Resolver for StaticResolver {
    fn resolve(&self, hostname: &str) -> Result<Vec<IpAddr>, DnsError> {
        if hostname.is_empty() {
            return Err(DnsError::InvalidName(hostname.to_string()));
        }
        self.entries
            .get(&hostname.to_lowercase())
            .cloned()
            .ok_or_else(|| DnsError::NotFound(hostname.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};

    #[test]
    fn test_static_resolver_returns_configured_address() {
        // Arrange
        let mut r = StaticResolver::new();
        let addr = IpAddr::V4(Ipv4Addr::new(93, 184, 216, 34));
        r.add("example.com", vec![addr]);
        // Act
        let result = r.resolve("example.com").unwrap();
        // Assert
        assert_eq!(result, vec![addr]);
    }

    #[test]
    fn test_missing_hostname_returns_not_found() {
        // Arrange
        let r = StaticResolver::new();
        // Act
        let result = r.resolve("unknown.local");
        // Assert
        assert!(matches!(result, Err(DnsError::NotFound(_))));
    }

    #[test]
    fn test_empty_hostname_returns_invalid() {
        // Arrange
        let r = StaticResolver::new();
        // Act
        let result = r.resolve("");
        // Assert
        assert!(matches!(result, Err(DnsError::InvalidName(_))));
    }
}
