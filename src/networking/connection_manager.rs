use crate::*;

#[derive(Resource)]
pub struct WebsocketClient {
    //pub ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    // pub sender: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    // pub receiver: Arc<Mutex<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
    pub runtime: runtime::Runtime,
    pub state_updates: Arc<DashMap<u8, Vec<Box<Vec<u8>>>>>,
    pub server_connection: Option<ServerConnection>,
    pub connection_events: network_plugin::SyncChannel<ConnectionEvent>, // Connected, Disconnected, Error.
    pub created_new_connection_events: network_plugin::SyncChannel<(WebSocketStream<MaybeTlsStream<TcpStream>>, SocketAddr)>, // Channel can send websocket and a string holding the current address its connected to.
    //pub received_messages_hashmap: Arc<HashMap<>>,
}
impl WebsocketClient {
    pub fn new() -> Self {
        println!("Websocket client created.");
        Self {
            state_updates: Arc::new(DashMap::new()),
            runtime: runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("Failed to build tokio runtime for websocket client."),
            server_connection: None,
            connection_events: SyncChannel::new(),
            created_new_connection_events: SyncChannel::new(),
        }
    }
    pub fn connect(&mut self, addr: SocketAddr) {
        println!("Attemting to create websocket connection.");
        self.disconnect();
        let connection_events = self.connection_events.sender.clone();
        let created_new_connection_events = self.created_new_connection_events.sender.clone();

        self.runtime.spawn(async move {
            // Connecting to the websocket.
            let url = url::Url::parse(format!("{}{}", "ws://", addr).as_str()).unwrap();
            let (ws_stream, _) = connect_async(url)
                .await
                .expect("Failed to connect to the server");

            // Trying to send the websocket & the address to the WebsocketClient struct for storage.
            match created_new_connection_events.send((ws_stream, addr)) {
                Ok(_) => {
                    connection_events.send(ConnectionEvent::Connected).unwrap();
                    println!("Successfully connected to websocket at address {}", addr.to_string());
                },
                Err(err) => {
                    connection_events.send(ConnectionEvent::Error).unwrap();
                    println!("Could not initiate connection: {}", err);
                }
            }
        });
    }
    pub fn disconnect(&mut self) {
        if let Some(server_conn) = self.server_connection.take() {
            server_conn.stop();

            let _ = self
                .connection_events
                .sender
                .send(ConnectionEvent::Disconnected);
        }
    }
    /// Send a message to the connected server, returns `Err(NetworkError::NotConnected)` if
    /// the connection hasn't been established yet
    pub fn send_message<T: quick_protobuf::MessageWrite> (&self, message_data: T, header: u8) -> Result<(), NetworkError> {
        let server_connection = match self.server_connection.as_ref() {
            Some(server) => server,
            None => return Err(NetworkError::NotConnected),
        };
        //println!("Serializing package to send to {}", server_connection.address.to_string());
        let packet = proto_serialize(message_data, header);

        match server_connection.message_sender.send(packet) {
            Ok(_) => (),
            Err(_) => {
                return Err(NetworkError::NotConnected);
            }
        }

        Ok(())
    }
}