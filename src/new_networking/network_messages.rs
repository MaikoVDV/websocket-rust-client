use crate::*;

pub struct GameStateUpdateMessage {}

impl NetworkMessage for GameStateUpdateMessage {
    const HEADER: &'static u8 = &3;
}