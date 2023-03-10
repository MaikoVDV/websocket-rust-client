use crate::*;

#[derive(Resource, Debug)]
pub struct GameWorld {
    pub players: HashMap<u32, proto_all::Entity>,
    pub bodies: HashMap<u32, proto_all::Body>,
}

impl GameWorld {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            bodies: HashMap::new(),
        }
    }
}
