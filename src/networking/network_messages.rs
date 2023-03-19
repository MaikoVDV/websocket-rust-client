use crate::*;

pub struct GameStateUpdateMessage {}
impl NetworkMessage for GameStateUpdateMessage {
    const HEADER: &'static u8 = &10;
}

pub struct InitialStateMessage{}
impl NetworkMessage for InitialStateMessage {
    const HEADER: &'static u8 = &11;
}

pub struct ClientConnectMessage{}
impl NetworkMessage for ClientConnectMessage {
    const HEADER: &'static u8 = &0;
}
pub struct ClientDisconnectMessage{}
impl NetworkMessage for ClientDisconnectMessage {
    const HEADER: &'static u8 = &1;
}
