use crate::*;

pub fn add_entities(mut game_world: ResMut<game_world::GameWorld>, entities: Vec<proto_all::Entity>) {
    for entity in entities.iter() {
        game_world.players.insert(entity.id, entity.to_owned());
    }
}