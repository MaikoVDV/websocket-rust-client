use crate::{proto::proto_all::GameState, *};

use futures_util::{stream::SplitStream, StreamExt};
use tokio::net::TcpStream;
//use tokio::task::unconstrained;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

//use futures_util::FutureExt;
use quick_protobuf::{BytesReader, MessageRead};

// Runs on a separate thread.
// Listens for changes in GameInput, and then transmits them via the websocket.
pub async fn get_game_state(
    state_sender: UnboundedSender<GameState>,
    mut ws_stream: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
) {
    println!("Websocket listener thread created. Entering loop.");
    let mut amount = 0;
    while let Some(msg) = ws_stream.next().await {
        if let Ok(msg) = msg {
            println!("Received message");
            if msg.is_binary() {
                let mut msg = msg.into_data();
                if msg.len() <= 0 {
                    info!("Received a message with a length of 0 or less. Not processing.");
                    break;
                }
                let header = msg.remove(0);
                let mut reader = BytesReader::from_bytes(&msg);
                if header == 0 {
                    if let Ok(state) = GameState::from_reader(&mut reader, &msg) {
                        amount += 1;
                        println!(
                            "Received state. Amount of entities: {}, Amount of bodies: {}, Amount of states received: {}",
                            state.entities.len(),
                            state.bodies.len(),
                            amount.to_string()
                        );
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
pub async fn test_func(
    state_sender: UnboundedSender<GameState>,
    mut ws_stream: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
) {
    let ws_listener = {
        ws_stream.for_each(|received_data| async {
            let mut msg = received_data.unwrap();
            println!("Received a message.");
            if msg.is_binary() {
                let mut msg = msg.into_data();
                if msg.len() <= 0 {
                    println!("Received a message with a length of 0 or less. Not processing.");
                    return;
                }
                let header = msg.remove(0);
                let mut reader = BytesReader::from_bytes(&msg);
                if header == 0 {
                    if let Ok(state) = GameState::from_reader(&mut reader, &msg) {
                        println!(
                            "Received state. Amount of entities: {}, Amount of bodies: {}",
                            state.entities.len(),
                            state.bodies.len()
                        );
                        let _ = state_sender.send(state);
                    }
                }
            } else if msg.is_close() {
                //break; // When we break, we disconnect.
            }
        })
    };
    ws_listener.await;
}
