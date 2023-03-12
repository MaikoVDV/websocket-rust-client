use crate::*;

// pub fn listen(tokio_channels: ResMut<TokioChannels>, mut websocket_stream: ResMut<connection_manager::WebsocketStream>) {
//     // let pool = IoTaskPool::get();
//     // let cc = comm_channels.tx.clone();
//     // let task = pool.spawn(async move {
//     //     let api_response_text = reqwest::get("http://localhost:8081/test")
//     //         .await
//     //         .unwrap()
//     //         .text()
//     //         .await
//     //         .unwrap();
//     //     cc.try_send(api_response_text);
//     // });
//     let task_pool = IoTaskPool::get();

//     // let listen_task = task_pool.spawn(async move {
//     //     println!("Websocket listener thread created. Entering loop.");
//     //     let mut _amount = 0;
//     //     while let Some(msg) = websocket_stream.stream.next().await {
//     //         println!("Received something");
//     //         if let Ok(msg) = msg {
//     //             if msg.is_binary() {
//     //                 let mut msg = msg.into_data();
//     //                 if msg.len() <= 0 {
//     //                     info!("Received a message with a length of 0 or less. Not processing.");
//     //                     continue;
//     //                 }
//     //                 // Header should be 0. Specifies that this is state sent from the server.
//     //                 let header = msg.remove(0);
//     //                 let mut reader = BytesReader::from_bytes(&msg);
//     //                 match header {
//     //                     0 => {
//     //                         if let Ok(state) = proto_all::GameState::from_reader(&mut reader, &msg) {
//     //                             println!(
//     //                                 "Received full state: \nAmount of entities: {},\nAmount of bodies: {},\nAmount of states received: {}",
//     //                                 state.entities.len(),
//     //                                 state.bodies.len(),
//     //                                 _amount.to_string()
//     //                             );
//     //                             //let _ = state_update_sender.send(state);
//     //                         }
//     //                     }
//     //                     1 => {
//     //                         println!("Received a message with header 1, which is meant for ClientInput messages. 
//     //                                 This is probably a bug in the server.");
//     //                     }
//     //                     2 => {
//     //                         if let Ok(client_join) =
//     //                             proto_all::ClientJoined::from_reader(&mut reader, &msg)
//     //                         {
//     //                             println!(
//     //                                 "Received Message::ClientJoin. The client's id is {}",
//     //                                 client_join.client_id.to_string()
//     //                             );
//     //                         }
//     //                     }
//     //                     3 => {
//     //                         if let Ok(state_update) =
//     //                             proto_all::GameStateUpdate::from_reader(&mut reader, &msg)
//     //                         {
//     //                             _amount += 1;
//     //                             //println!(
//     //                             //    "Received state update: \nAmount of entities: {},\nAmount of bodies: {}",
//     //                             //    state_update.entities.len(),
//     //                             //    state_update.bodies.len(),
//     //                             //);
//     //                             if state_update.entities.len() <= 0 {
//     //                                 continue;
//     //                             }
//     //                             let first_entity = state_update.entities.get(0).unwrap();
//     //                             println!(
//     //                                 "Received state update. Information on 1st entity:\nid: {}\nx: {}\ny: {}\npressed: {}",
//     //                                 first_entity.id,
//     //                                 first_entity.x,
//     //                                 first_entity.y,
//     //                                 first_entity.pressed,
//     //                             );
//     //                             let _ = state_update_sender.send(state_update);
//     //                         }
//     //                     }
//     //                     _ => {
//     //                         error!(
//     //                             "Received message with invalid header: {}   | Discarding.",
//     //                             header.to_string()
//     //                         );
//     //                     }
//     //                 }
//     //             } else if msg.is_close() {
//     //                 info!("Websocket connection closed.");
//     //                 break; // When we break, we disconnect.
//     //             }
//     //         } else {
//     //             break; // When we break, we disconnect.
//     //         }
//     //     }
//     // });
// }
pub async fn listen(
    mut ws_receiver: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    state_updates: Arc<DashMap<u8, Vec<Box<Vec<u8>>>>>
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
                            if let Ok(state) = proto_all::GameState::from_reader(&mut reader, &msg) {
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
                                // println!(
                                //    "Received state update: \nAmount of players: {},\nAmount of bodies: {}",
                                //    state_update.players.len(),
                                //    state_update.bodies.len(),
                                // );
                                //println!("Received and deserialized GameStateUpdate");
                                if state_update.players.len() <= 0 {
                                    continue;
                                }
                                let _first_player = state_update.players.get(0).unwrap();
                                // println!(
                                //     "Received state update. Information on 1st player:\nid: {}\nx: {}\ny: {}\npressed: {}",
                                //     _first_player.id,
                                //     _first_player.x,
                                //     _first_player.y,
                                //     _first_player.pressed,
                                // );
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

    events.send_batch(
        messages
            .drain(..)
            .map(|msg| {
                NetworkData {
                    source: ws_client.server_connection.as_ref()
                        .map(|s_connection| s_connection.address)
                        .unwrap_or_else(|| SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0)),
                    header: T::HEADER.to_owned(), // Prob dont need this field
                    data: *msg,
                }
            }),
    );
}