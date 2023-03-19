use crate::*;

#[derive(Component)]
pub struct Player {
    pub input_queue: GameInputQueue, // Stores inputs made this tick for sending
    pub position: Position,          // Position is x, y in f32
    pub pressed: bool,               // If the spacebar is pressed or not
    pub server_id: u32,              // Identifier used by the server.
}

impl Player {
    pub fn new(server_id: u32, x: f32, y: f32) -> Player {
        Player {
            input_queue: GameInputQueue::new(x, y, false),
            position: Position { x, y },
            pressed: false,
            server_id,
        }
    }

    pub fn update_with_proto(&mut self, updated_data: generic_protobufs::Player) {
        self.position.x = updated_data.x;
        self.position.y = updated_data.y;
        self.pressed = updated_data.pressed;
    }
}

#[derive(Component)]
pub struct GameInputQueue {
    pub position: Position,
    pub pressed: bool
}
impl GameInputQueue {
    pub fn new(x: f32, y: f32, pressed: bool) -> GameInputQueue {
        GameInputQueue {
            position: Position {
                x: x,
                y: y,
            },
            pressed: pressed
        }
    }
}
// Only put on the player that is being controlled by this client
#[derive(Component)]
pub struct ControlledPlayer {}
