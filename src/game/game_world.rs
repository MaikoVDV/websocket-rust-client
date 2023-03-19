use crate::*;

#[derive(Resource, Debug)]
pub struct GameWorld {
    pub players: HashMap<u32, generic_protobufs::Player>,
    pub bodies: HashMap<u32, generic_protobufs::Body>,
}

impl GameWorld {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            bodies: HashMap::new(),
        }
    }
}
