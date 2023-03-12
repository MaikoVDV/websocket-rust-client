use crate::*;

pub fn listen_for_state_changes(mut game_world: ResMut<game_world::GameWorld>, mut data_events: EventReader<NetworkData>) {
    if data_events.len() > 0 {
        println!("Amount of state update events: {}", data_events.len());
    }
    for update_event in data_events.iter() {
        let mut reader = BytesReader::from_bytes(&update_event.data);
        let state_update = proto_all::GameStateUpdate::from_reader(&mut reader, &update_event.data).unwrap_or_default();
        println!("Got a state update:\nAmount of players updated: {}\nAmount of bodies updated: {}", state_update.entities.len(), state_update.bodies.len());
    }
    // let new_state = match data_event.iter() {
    //     Ok(state) => state,
    //     Err(err) => {
    //         return;
    //     }
    // };
    // println!("Got some state changes while listenting to a channel:\nAmount of players: {}\nAmount of bodies: {}", new_state.entities.len(), new_state.bodies.len());
}
// pub fn add_entities(
//     mut game_world: ResMut<game_world::GameWorld>,
//     entities: Vec<proto_all::Entity>,
// ) {
//     for entity in entities.iter() {
//         game_world.players.insert(entity.id, entity.to_owned());
//     }
// }