use crate::*;

pub fn listen_for_state_updates(
    mut data_events: EventReader<NetworkData>,
    mut commands: Commands,
    mut current_players: Query<&mut components::player::Player>,
) {
    // Filters the events to only contain messages of type GameStateUpdate
    let update_events = data_events.iter().filter(|event| event.header == 3);

    // Iterate over GameStateUpdates and update local state with the data
    for update_event in update_events {
        let mut reader = BytesReader::from_bytes(&update_event.data);
        let state_update = proto_all::GameStateUpdate::from_reader(&mut reader, &update_event.data)
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
}

pub fn listen_for_initial_state(
    mut data_events: EventReader<NetworkData>,
    mut ws_client: ResMut<WebsocketClient>,
    mut commands: Commands,
) {
    // Filters the events to only contain messages of type GameStateUpdate
    let initial_state_events = data_events.iter().filter(|event| event.header == 4);

    // Iterate over GameStateUpdates and update local state with the data
    for init_state_event in initial_state_events {
        println!("Received InitialState message. Updating local world.");

        // Deserializing InitialState, which holds client_id and GameStateUpdate (serialized)
        let mut init_state_reader = BytesReader::from_bytes(&init_state_event.data);
        let init_state =
            proto_all::InitialState::from_reader(&mut init_state_reader, &init_state_event.data)
                .unwrap_or_default();
        // Deserializing GameStateUpdate
        //        let mut state_update_reader = BytesReader::from_bytes(&init_state.full_state);
        //        let state_update =
        //            proto_all::GameStateUpdate::from_reader(&mut state_update_reader, &)
        //                .unwrap_or_default();

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
            println!("After nested for");
            commands.spawn(components::player::Player::new(
                player.id, player.x, player.y,
            ));
        }
    }
}
