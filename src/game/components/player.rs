use crate::*;

#[derive(Component)]
pub struct Player {
    pub position: Position,
    pub server_id: u32, // Identifier used by the server.
}

impl Player {
    pub fn new(server_id: u32, x: f32, y: f32) -> Player {
        Player {
            position: Position{
                x,
                y
            },
            server_id,
        }
    }
    pub fn update_with_proto(&mut self, updated_data: proto_all::Player) {
        self.position.x = updated_data.x;
        self.position.y = updated_data.y;
    }
}