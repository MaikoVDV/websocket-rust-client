mod proto; // Converts data stored in memory to messages to be sent via websocket
mod connection; // Opens & manages the websocket
mod receive_state; // Creates thread that listens for state changes over the websocket
mod broadcast_gameinput; // Creates thread that broadcasts new GameInputs to the server via the websocket

use bevy::prelude::*;

use quick_protobuf::{Writer};

use proto::proto_all::*;
use connection::init_websocket_connection;
use broadcast_gameinput::broadcast_game_input;
use receive_state::get_game_state;

use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};


const PORT: &str = "8080";
//const TIMESTEP: f32 = 1.0 / 60.0; // 60tps server

#[tokio::main]
async fn main() {
    // Creating a websocket for communication with the server.
    let ws_stream = init_websocket_connection().await;
    let (ws_sender, mut ws_receiver) = ws_stream.split();

    println!("Creating channel for gameinput communication between threads.");
    let (input_sender, input_receiver) = mpsc::unbounded_channel::<GameInput>();
    tokio::spawn(broadcast_game_input(input_receiver, ws_sender));

    println!("Creating channel for state communication between threads.");
    let (state_sender, state_receiver) = mpsc::unbounded_channel::<GameState>();
    tokio::spawn(get_game_state(state_sender, ws_receiver));

    println!("Starting bevy app");
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GameInputSenderResource {sender: input_sender})
        .insert_resource(StateReceiverResource {receiver: state_receiver})
        .add_system(handle_input)
        .run();

    // App::new().run() is blocking. Do not write code after this point.
}

fn handle_input(
    keys: Res<Input<KeyCode>>,
    //windows: Res<Windows>, // Used to get mouse position
    mut cursor_evr: EventReader<CursorMoved>,
    input_sender: Res<GameInputSenderResource>
) {
    if keys.just_pressed(KeyCode::Space) {
        println!("<Space> was pressed");
        println!("Channel closed: {}", input_sender.sender.is_closed());
        let input = GameInput { x: 0.0, y: 0.0, pressed: true };
        //input_sender.sender.send(input).unwrap();
        match input_sender.sender.send(input) {
            Ok(()) => (),
            Err(err) => eprintln!("Error: Transmitting keyboardinput through channel failed: {}", err)
        };
    }
    for cursor_event in cursor_evr.iter() {
        let input = GameInput { x: cursor_event.position.x, y: cursor_event.position.y, pressed: true };
        match input_sender.sender.send(input) {
            Ok(()) => (),
            Err(err) => eprintln!("Error: Transmitting mouse input through channel failed: {}", err)
        };
    }
}


// Our helper method which will read data from stdin and send it along the
// sender provided.
// async fn read_stdin(tx: futures_channel::mpsc::UnboundedSender<Message>) {
//     let mut stdin = tokio::io::stdin();
//     loop {
//         let mut buf = vec![0; 1024];
//         let n = match stdin.read(&mut buf).await {
//             Err(_) | Ok(0) => break,
//             Ok(n) => n,
//         };
//         buf.truncate(n);
//         tx.unbounded_send(Message::binary(buf)).unwrap();
//     }
// }
#[derive(Resource, Debug)]
pub struct GameInputSenderResource {
    pub sender: UnboundedSender<GameInput>,
}

#[derive(Resource, Debug)]
pub struct StateReceiverResource {
    pub receiver: UnboundedReceiver<proto::proto_all::GameState>,
}