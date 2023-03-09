use crate::*;

use futures_util::stream::SplitStream;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

use quick_protobuf::{BytesReader, MessageRead};

// Runs on a separate thread.
// Listens for changes in GameInput, and then transmits them via the websocket.
pub async fn get_game_state(
    state_sender: mpsc::UnboundedSender<GameState>,
    mut ws_stream: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
) {
    println!("Websocket listener thread created. Entering loop.");
    let mut _amount = 0;
    while let Some(msg) = ws_stream.next().await {
        if let Ok(msg) = msg {
            if msg.is_binary() {
                let mut msg = msg.into_data();
                if msg.len() <= 0 {
                    info!("Received a message with a length of 0 or less. Not processing.");
                    break;
                }
                // Header should be 0. Specifies that this is state sent from the server.
                let header = msg.remove(0);
                let mut reader = BytesReader::from_bytes(&msg);
                match header {
                    0 => {
                        if let Ok(state) = GameState::from_reader(&mut reader, &msg) {
                            _amount += 1;
                            info!(
                                "Received state: \nAmount of entities: {},\nAmount of bodies: {},\nAmount of states received: {}",
                                state.entities.len(),
                                state.bodies.len(),
                                _amount.to_string()
                            );
                            let _ = state_sender.send(state);
                        }
                    }
                    1 => {
                        println!("Received a message with header 1, which is meant for ClientInput messages. 
                                 This is probably a bug in the server.");
                    }
                    2 => {
                        if let Ok(client_join) = ClientJoined::from_reader(&mut reader, &msg) {
                            println!(
                                "Received Message::ClientJoin. The client's id is {}",
                                client_join.id.to_string()
                            );
                        }
                    }
                    _ => {
                        error!(
                            "Received message with invalid header: {}   | Discarding.",
                            header.to_string()
                        );
                    }
                }
            } else if msg.is_close() {
                info!("Websocket connection closed.");
                break; // When we break, we disconnect.
            }
        } else {
            break; // When we break, we disconnect.
        }
    }
}
