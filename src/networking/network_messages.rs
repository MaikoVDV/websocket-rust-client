use crate::*;

pub struct GameStateUpdateMessage {}
impl NetworkMessage for GameStateUpdateMessage {
    const HEADER: &'static u8 = &3;
}


pub struct InitialStateMessage{}
impl NetworkMessage for InitialStateMessage {
    const HEADER: &'static u8 = &4;
}
