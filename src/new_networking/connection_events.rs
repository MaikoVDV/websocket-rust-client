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

// impl<T> NetworkData<T> {
//     pub fn new(source: SocketAddr, inner: T) -> Self {
//         Self { source, inner }
//     }

//     /// The source of this network data
//     pub fn source(&self) -> SocketAddr {
//         self.source
//     }

//     /// Get the inner data out of it
//     pub fn into_inner(self) -> T {
//         self.inner
//     }
// }