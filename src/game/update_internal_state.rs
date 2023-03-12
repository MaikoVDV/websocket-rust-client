use crate::*;

pub fn listen_for_state_changes(
    mut data_events: EventReader<NetworkData>,
    mut commands: Commands,
    mut current_players: Query<&mut components::player::Player>,
) {
    for update_event in data_events.iter() {
        let mut reader = BytesReader::from_bytes(&update_event.data);
        let state_update = proto_all::GameStateUpdate::from_reader(&mut reader, &update_event.data).unwrap_or_default();
        // //println!("Got a state update:\nAmount of players updated: {}\nAmount of bodies updated: {}", state_update.entities.len(), state_update.bodies.len());
        println!("Amount of players currently in Bevy world: {}", current_players.iter().len());
        'player_update: for updated_player in &state_update.players {
            println!("Player {} updated:     x: {}, y: {}, pressed: {}", updated_player.id, updated_player.x, updated_player.y, updated_player.pressed);

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
                updated_player.y
            ));
        }
    }
}