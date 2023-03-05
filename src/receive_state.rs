use crate::{*, proto::proto_all::GameState};

use futures_util::{StreamExt, stream::SplitStream};
use tokio::net::TcpStream;
//use tokio::task::unconstrained;
use tokio_tungstenite::{WebSocketStream, MaybeTlsStream};

//use futures_util::FutureExt;
use quick_protobuf::{BytesReader, MessageRead};


// Runs on a separate thread.
// Listens for changes in GameInput, and then transmits them via the websocket.
pub async fn get_game_state(
    state_sender: UnboundedSender<GameState>,
    mut ws_stream: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
) {
    println!("Websocket listener thread created. Entering loop.");
    while let Some(msg) = ws_stream.next().await {
        println!("Received some kind of something through the websocket.");
        if let Ok(msg) = msg {
            println!("Received message");
            if msg.is_binary() {
                let mut msg = msg.into_data();
                if msg.len() <= 0 {
                    info!("Received a message with a length of 0 or less. Not processing.");
                    break
                }
                let header = msg.remove(0);
                let mut reader = BytesReader::from_bytes(&msg);
                if header == 0 {
                    if let Ok(state) = GameState::from_reader(&mut reader, &msg) {
                        println!("Received state. Amount of entities: {}, Amount of bodies: {}", state.entities.len(), state.bodies.len());
                        let _ = state_sender.send(state);
                    }
                }
            } else if msg.is_close() {
                break; // When we break, we disconnect.
            }
        } else {
            break; // When we break, we disconnect.
        }
    }
    println!("Websocket connection has closed.");
}