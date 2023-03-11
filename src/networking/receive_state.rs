use crate::*;

use futures_util::stream::SplitStream;

use quick_protobuf::{BytesReader, MessageRead};

// Runs on a separate thread.
// Listens for changes in GameInput, and then transmits them via the websocket.
pub async fn get_game_state(
    state_sender: mpsc::UnboundedSender<proto_all::GameState>,
    state_update_sender: mpsc::UnboundedSender<proto_all::GameStateUpdate>,
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
                    continue;
                }
                // Header should be 0. Specifies that this is state sent from the server.
                let header = msg.remove(0);
                let mut reader = BytesReader::from_bytes(&msg);
                match header {
                    0 => {
                        if let Ok(state) = proto_all::GameState::from_reader(&mut reader, &msg) {
                            println!(
                                "Received full state: \nAmount of entities: {},\nAmount of bodies: {},\nAmount of states received: {}",
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
                        if let Ok(client_join) =
                            proto_all::ClientJoined::from_reader(&mut reader, &msg)
                        {
                            println!(
                                "Received Message::ClientJoin. The client's id is {}",
                                client_join.client_id.to_string()
                            );
                        }
                    }
                    3 => {
                        if let Ok(state_update) =
                            proto_all::GameStateUpdate::from_reader(&mut reader, &msg)
                        {
                            _amount += 1;
                            //println!(
                            //    "Received state update: \nAmount of entities: {},\nAmount of bodies: {}",
                            //    state_update.entities.len(),
                            //    state_update.bodies.len(),
                            //);
                            if state_update.entities.len() <= 0 {
                                continue;
                            }
                            let first_entity = state_update.entities.get(0).unwrap();
                            println!(
                                "Received state update. Information on 1st entity:\nid: {}\nx: {}\ny: {}\npressed: {}",
                                first_entity.id,
                                first_entity.x,
                                first_entity.y,
                                first_entity.pressed,
                            );
                            let _ = state_update_sender.send(state_update);
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
