use crate::*;

#[derive(Debug)]
pub enum ConnectionEvent {
    Connected,
    Disconnected,
    Error,
}

// NetworkData is sent over the network.
#[derive(Debug)]
pub struct NetworkData {
    pub source: SocketAddr,
    pub header: u8,
    pub data: Vec<u8>,
}