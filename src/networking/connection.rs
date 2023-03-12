use crate::*;

// Stores information about the connection with the server.
// Struct is constructed by WebsocketConnection in connect().
#[derive(Debug)]
pub struct ServerConnection {
    pub address: SocketAddr, // The address of the server currently connected to
    pub listen_task: JoinHandle<()>, // Tokio future that for messages sent by the server
    pub broadcast_task: JoinHandle<()>, // Tokio future that sends data to the server (like GameInputs, Pings, etc.)
    pub message_sender: mpsc::UnboundedSender<Vec<u8>>, // Use this to send messages to the server.
}

// After the client has connected to the server, this function will run.
pub fn handle_connection_event(
    mut ws_client: ResMut<WebsocketClient>,
) {
    let (connection, server_address) =
        match ws_client.created_new_connection_events.receiver.try_recv() {
            Ok(event) => event,
            Err(_err) => {
                return;
            }
        };
    
    let (send_socket, read_socket) = connection.split(); // The actual websocket object, split in two to handle listening and broadcasting separately
    let state_updates = ws_client.state_updates.clone(); // A vector containing vectors of u8's, wrapped in an Arc. In other words, an async array of packets.
    let (send_message, recv_message) = mpsc::unbounded_channel::<Vec<u8>>();

    ws_client.server_connection = Some(ServerConnection {
        address: server_address,
        message_sender: send_message,

        listen_task: ws_client.runtime.spawn(async move {
            let read_socket = read_socket; // Object to listen to
            let state_updates = state_updates; // Will push onto this vector when a message is received
            listen(read_socket, Arc::clone(&state_updates)).await;
        }),

        broadcast_task: ws_client.runtime.spawn(async move {
            println!("Broadcasting thread has been created.");
            let send_socket = send_socket; // Actual websocket that messages are sent to
            let recv_message = recv_message; // mpsc channel with messages queued up for sending.
            broadcast(send_socket, recv_message).await;
        }),
    })
}

impl ServerConnection {
    pub fn stop(self) {
        self.listen_task.abort();
        self.broadcast_task.abort();
    }
}