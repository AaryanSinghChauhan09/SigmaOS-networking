//! Connection state machine for SigmaOS networking.
use crate::addr::Endpoint;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnState {
    Disconnected,
    Connecting,
    Connected,
    Closing,
    Failed,
}

impl fmt::Display for ConnState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ConnState::Disconnected => "disconnected",
            ConnState::Connecting   => "connecting",
            ConnState::Connected    => "connected",
            ConnState::Closing      => "closing",
            ConnState::Failed       => "failed",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug)]
pub enum ConnError {
    InvalidTransition { from: ConnState, to: ConnState },
    AlreadyConnected,
    NotConnected,
}

impl fmt::Display for ConnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnError::InvalidTransition { from, to } =>
                write!(f, "invalid transition {from} -> {to}"),
            ConnError::AlreadyConnected => write!(f, "already connected"),
            ConnError::NotConnected => write!(f, "not connected"),
        }
    }
}

/// A managed connection with explicit state transitions.
/// Separation of Mechanism (state machine) & Policy (application logic).
#[derive(Debug)]
pub struct Connection {
    endpoint: Endpoint,
    state: ConnState,
    bytes_sent: u64,
    bytes_recv: u64,
}

impl Connection {
    pub fn new(endpoint: Endpoint) -> Self {
        Self { endpoint, state: ConnState::Disconnected, bytes_sent: 0, bytes_recv: 0 }
    }

    // ── Queries ──────────────────────────────────────────────────────────
    pub fn state(&self) -> ConnState { self.state }
    pub fn endpoint(&self) -> &Endpoint { &self.endpoint }
    pub fn is_connected(&self) -> bool { self.state == ConnState::Connected }
    pub fn bytes_sent(&self) -> u64 { self.bytes_sent }
    pub fn bytes_recv(&self) -> u64 { self.bytes_recv }

    // ── Commands ─────────────────────────────────────────────────────────
    pub fn connect(&mut self) -> Result<(), ConnError> {
        match self.state {
            ConnState::Disconnected | ConnState::Failed => {
                self.state = ConnState::Connecting;
                self.state = ConnState::Connected; // Simulated immediate connect
                Ok(())
            }
            ConnState::Connected => Err(ConnError::AlreadyConnected),
            other => Err(ConnError::InvalidTransition { from: other, to: ConnState::Connected }),
        }
    }

    pub fn disconnect(&mut self) -> Result<(), ConnError> {
        if self.state != ConnState::Connected {
            return Err(ConnError::NotConnected);
        }
        self.state = ConnState::Closing;
        self.state = ConnState::Disconnected;
        Ok(())
    }

    pub fn record_send(&mut self, bytes: u64) {
        self.bytes_sent += bytes;
    }

    pub fn record_recv(&mut self, bytes: u64) {
        self.bytes_recv += bytes;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_conn() -> Connection {
        Connection::new(Endpoint::loopback(8080))
    }

    #[test]
    fn test_initial_state_is_disconnected() {
        // Arrange / Act
        let conn = make_conn();
        // Assert
        assert_eq!(conn.state(), ConnState::Disconnected);
    }

    #[test]
    fn test_connect_changes_state() {
        // Arrange
        let mut conn = make_conn();
        // Act
        conn.connect().unwrap();
        // Assert
        assert!(conn.is_connected());
    }

    #[test]
    fn test_double_connect_returns_error() {
        // Arrange
        let mut conn = make_conn();
        conn.connect().unwrap();
        // Act
        let result = conn.connect();
        // Assert
        assert!(matches!(result, Err(ConnError::AlreadyConnected)));
    }

    #[test]
    fn test_disconnect_when_not_connected_returns_error() {
        // Arrange
        let mut conn = make_conn();
        // Act
        let result = conn.disconnect();
        // Assert
        assert!(matches!(result, Err(ConnError::NotConnected)));
    }

    #[test]
    fn test_bytes_tracking() {
        // Arrange
        let mut conn = make_conn();
        conn.connect().unwrap();
        // Act
        conn.record_send(512);
        conn.record_recv(1024);
        // Assert
        assert_eq!(conn.bytes_sent(), 512);
        assert_eq!(conn.bytes_recv(), 1024);
    }
}
