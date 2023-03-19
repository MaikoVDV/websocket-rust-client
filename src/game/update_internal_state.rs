use crate::*;

pub fn listen_for_network_events(
    mut network_events: EventReader<NetworkData>,
    mut commands: Commands,
    mut current_players: Query<&mut components::player::Player>,
    mut ws_client: ResMut<WebsocketClient>,
) {
    for event in network_events.iter() {
        let mut data_reader = BytesReader::from_bytes(&event.data);
        match event.header {
            10 => { // GameStateUpdate
                let state_update = state_messages::GameStateUpdate::from_reader(&mut data_reader, &event.data)
                .unwrap_or_default();
    
                'player_update: for updated_player in &state_update.players {
                    println!(
                        "Player {} updated:     x: {}, y: {}, pressed: {}",
                        updated_player.id, updated_player.x, updated_player.y, updated_player.pressed
                    );
        
                    for mut local_player in current_players.iter_mut() {
                        if local_player.server_id == updated_player.id {
                            local_player.update_with_proto(updated_player.to_owned());
                            continue 'player_update; // Moves to the next updated player that was received.
                        }
                    }
                    // If no player is found that matches the id of the updated player, then that new player should be added.
                    println!("After nested for");
                    commands.spawn(components::player::Player::new(
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
                match ws_client.server_connection.as_mut() {
                    Some(conn) => conn.add_client_id(init_state.client_id),
                    None => (),
                }
                println!(
                    "Client id: {}",
                    ws_client
                        .server_connection
                        .as_mut()
                        .expect("Failed to unwrap server_connection")
                        .client_id
                        .expect("Failed to unwrap client_id")
                );
                for player in &state_update.players {
                    println!(
                        "Player {} data:     x: {}, y: {}, pressed: {}",
                        player.id, player.x, player.y, player.pressed
                    );
        
                    // If no player is found that matches the id of the updated player, then that new player should be added.
                    println!("Adding player {} from initial state", player.id);
                    commands.spawn(components::player::Player::new(
                        player.id, player.x, player.y,
                    ));
                }
            }
            0 => { // ClientConnect

            }
            _ => ()
        }
    }
}