use crate::*;

pub async fn listen(
    mut ws_receiver: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    state_updates: Arc<DashMap<u8, Vec<Box<Vec<u8>>>>>,
) {
    loop {
        let mut _amount = 0;
        while let Some(received_data) = ws_receiver.next().await {
            //println!("Received something");
            if let Ok(msg) = received_data {
                if msg.is_binary() {
                    let mut msg = msg.into_data();
                    if msg.len() <= 0 {
                        info!("Received a message with a length of 0 or less. Not processing.");
                        continue;
                    }
                    // Header should be 0. Specifies that this is state sent from the server.
                    let header = msg.remove(0);
                    let mut reader = BytesReader::from_bytes(&msg);
                    //println!("Received a binary message with header {}", header);
                    match header {
                        0 => {
                            if let Ok(state) = proto_all::GameState::from_reader(&mut reader, &msg)
                            {
                                println!(
                                    "Received full state: \nAmount of players: {},\nAmount of bodies: {},\nAmount of states received: {}",
                                    state.players.len(),
                                    state.bodies.len(),
                                    _amount.to_string()
                                );
                                //let _ = state_update_sender.send(state);
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
                                if state_update.players.len() <= 0 {
                                    continue;
                                }
                                let _first_player = state_update.players.get(0).unwrap();
                                match state_updates.get_mut(&header) {
                                    Some(mut updates) => updates.push(Box::new(msg)),
                                    None => {
                                        error!(
                                            "Could not find existing entries for message kinds: {:?}",
                                            state_update
                                        );
                                    }
                                }
                            }
                        }
                        4 => {
                            // Initial state
                            if let Ok(initial_state) =
                                proto_all::InitialState::from_reader(&mut reader, &msg)
                            {
                                match state_updates.get_mut(&header) {
                                    Some(mut init_states) => init_states.push(Box::new(msg)),
                                    None => {
                                        error!(
                                            "Could not find existing entries for message kind: {:?}",
                                            initial_state
                                        );
                                    }
                                }
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
}

// When listen() adds a new message to ws_client, this system reads that and sends an event with the data.
// Then, game logic can react to those events and extract data from them.
pub fn send_event_for_message<T: network_plugin::NetworkMessage>(
    ws_client: ResMut<WebsocketClient>,
    mut events: EventWriter<NetworkData>,
) {
    let mut messages = match ws_client.state_updates.get_mut(&T::HEADER) {
        Some(messages) => messages,
        None => return,
    };

    events.send_batch(messages.drain(..).map(|msg| {
        NetworkData {
            source: ws_client
                .server_connection
                .as_ref()
                .map(|s_connection| s_connection.address)
                .unwrap_or_else(|| SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0)),
            header: T::HEADER.to_owned(), // Prob dont need this field
            data: *msg,
        }
    }));
}
