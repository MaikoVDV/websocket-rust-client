use crate::*;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub server_id: components::generic::ServerID,
    pub player: Player,
    pub sprite_bundle: SpriteBundle,
}
impl PlayerBundle {
    pub fn new(server_id: u32, texture: Handle<Image>) -> PlayerBundle {
        PlayerBundle {
            server_id: ServerID(server_id),
            player: Player::new(),
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    scale: Vec3::new(1.0, 1.0, 1.0),
                    ..default()
                },
                texture,
                ..default()
            }
        }
    }
}

#[derive(Component, Default)]
pub struct Player {
    //pub input_queue: GameInputQueue, // Stores inputs made this tick for sending
    //pub position: Position,          // Position is x, y in f32
    pub pressed: bool,               // If the spacebar is pressed or not
}

impl Player {
    pub fn new() -> Player {
        Player {
            //input_queue: GameInputQueue::new(x, y, false),
            //position: Position { x, y },
            pressed: false,
        }
    }

    pub fn update_with_proto(&mut self, updated_data: generic_protobufs::Player) {
        //self.sprite_bundle.transform.translation = Vec3::new(updated_data.x, updated_data.y, 0.0);
        self.pressed = updated_data.pressed;
        //self.sprite_bundle.transform.translation = Vec3::new(updated_data.x, updated_data.y, 0.0);
    }
}

#[derive(Component, Default)]
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
#[derive(Component, Debug)]
pub struct ControlledPlayer;
