use crate::*;

pub fn listen_for_network_events(
    mut network_events: EventReader<NetworkData>,
    mut commands: Commands,
    mut current_players: Query<(&mut components::player::Player, Entity)>, // Querying full entity along with Player component for despawning
    mut ws_client: ResMut<WebsocketClient>,
) {
    let server_conn = match ws_client.server_connection.as_mut() {
        Some(conn) => conn,
        None => return
    };

    // Buffer for new players
    let mut new_players: HashMap<u32, components::player::Player> = HashMap::new();

    for event in network_events.iter() {
        let mut data_reader = BytesReader::from_bytes(&event.data);
        match event.header {
            10 => { // GameStateUpdate
                let state_update = state_messages::GameStateUpdate::from_reader(&mut data_reader, &event.data)
                .unwrap_or_default();
    
                'player_update: for updated_player in &state_update.players {
                    // println!(
                    //     "Player {} updated:     x: {}, y: {}, pressed: {}",
                    //     updated_player.id, updated_player.x, updated_player.y, updated_player.pressed
                    // );
        
                    for (mut local_player, _) in current_players.iter_mut() {
                        if local_player.server_id == updated_player.id {
                            local_player.update_with_proto(updated_player.to_owned());
                            continue 'player_update; // Moves to the next updated player that was received.
                        }
                    }
                    // If no player is found that matches the id of the updated player, then that new player should be added.
                    new_players.insert(updated_player.id, components::player::Player::new(
                        updated_player.id,
                        updated_player.x,
                        updated_player.y,
                    ));
                }
            }
            11 => { // InitialState
                println!("Received InitialState message. Updating local world.");

                // Deserializing InitialState, which holds client_id and GameStateUpdate (serialized)
                let init_state =
                    state_messages::InitialState::from_reader(&mut data_reader, &event.data)
                        .unwrap_or_default();
                let state_update = init_state.full_state.unwrap_or_default();
                server_conn.add_client_id(init_state.client_id);
                println!(
                    "Client id: {}",
                    server_conn
                        .client_id
                        .expect("Failed to unwrap client_id")
                );
                for player in &state_update.players {
                    new_players.insert(player.id, components::player::Player::new(
                        player.id,
                        player.x,
                        player.y,
                    ));
                }
            }
            0 => { // ClientConnect
                println!("A new client has connected!");
                // A new client has connected. Add them to the game world.
                let connect_data = conn_event_messages::ClientConnect::from_reader(&mut data_reader, &event.data)
                .unwrap_or_default();

                new_players.insert(connect_data.client_id, components::player::Player {
                    server_id: connect_data.client_id,
                    ..default()
                });
                
            }
            1 => { // ClientDisconnect
                println!("A client has disconnected!");
                let disconnect_data = conn_event_messages::ClientDisconnect::from_reader(&mut data_reader, &event.data)
                .unwrap_or_default();
                if let Some((_, entity_id)) = current_players.iter().find(|&player| player.0.server_id == disconnect_data.client_id) {
                    println!("Client {} has left the game.", disconnect_data.client_id);
                    commands.entity(entity_id).despawn();
                } else {
                    eprintln!("Got a ClientDisconnect message, but couldn't find any players in the world with id {}.", disconnect_data.client_id);
                }
            }
            _ => ()
        }
    }
    // Clear the new_players HashMap and for each entry, check if they already exist in the game world.
    // HashMap uses the client's ID as the key, so a player can only exist one in this HashMap.
    new_players.drain().for_each(|new_player| {
        if let None = current_players.iter().find(|&player| player.0.server_id == new_player.1.server_id) {
            println!("Client {} has joined the game.", new_player.1.server_id);
            let player_bundle = components::player::Player::new(
                new_player.1.server_id,
                new_player.1.position.x,
                new_player.1.position.y,
            );
            let mut spawn_command = commands.spawn(player_bundle);
            if server_conn.client_id == Some(new_player.1.server_id) {
                spawn_command.insert(components::player::ControlledPlayer);
            }
        } else {
            eprintln!("Got a ClientConnect message but there is already a client in the game with that id ({})", new_player.1.server_id);
        }
    })
}