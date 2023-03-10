use crate::*;

pub fn listen_for_state_changes(tokio_channels: Res<TokioChannels>) {}
pub fn add_entities(
    mut game_world: ResMut<game_world::GameWorld>,
    entities: Vec<proto_all::Entity>,
) {
    for entity in entities.iter() {
        game_world.players.insert(entity.id, entity.to_owned());
    }
}

