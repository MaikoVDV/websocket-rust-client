mod broadcast_gameinput;
mod connection; // Opens & manages the websocket
mod proto; // Converts data stored in memory to messages to be sent via websocket
mod receive_state; // Creates thread that listens for state changes over the websocket // Creates thread that broadcasts new GameInputs to the server via the websocket

use bevy::{
    prelude::*,
    render::{
        settings::{Backends, WgpuSettings},
        RenderPlugin,
    },
};

use quick_protobuf::Writer;

use broadcast_gameinput::broadcast_game_input;
use connection::init_websocket_connection;
use proto::proto_all::*;
use receive_state::{get_game_state, test_func};

use futures_util::{future, SinkExt, StreamExt};
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
    // Creating a thread to handle sending GameInputs to the server.
    // Also, creating an mpsc channel to send the inputs from the Bevy app to the thread.
    println!("Creating channel for gameinput communication between threads.");
    let (input_sender, input_receiver) = mpsc::unbounded_channel::<GameInput>();
    tokio::spawn(broadcast_game_input(input_receiver, ws_sender));
    // Creating a thread to handle receiving new gamestates from the server.
    // Also, creating an mpsc channel to send the state from the thread to the main thread, and to
    // the Bevy app.
    println!("Creating channel for state communication between threads.");
    let (state_sender, state_receiver) = mpsc::unbounded_channel::<GameState>();
    tokio::spawn(get_game_state(state_sender, ws_receiver));
    //tokio::spawn(test_func(state_sender, ws_receiver));

    // Creating the Bevy app. This runs on the main thread, and is sync, so it's blocking.
    // Do not write code after this point, as it will not run until the Bevy app exits.
    println!("Starting bevy app");
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Websocket game client written with Bevy".into(),
                        resolution: (640.0, 360.0).into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(RenderPlugin {
                    wgpu_settings: WgpuSettings {
                        backends: Some(Backends::GL),
                        ..default()
                    },
                }),
        )
        .insert_resource(GameInputSenderResource {
            sender: input_sender,
        })
        .insert_resource(StateReceiverResource {
            receiver: state_receiver,
        })
        .add_system(handle_input)
        .run();
}

fn handle_input(
    keys: Res<Input<KeyCode>>,
    mut cursor_evr: EventReader<CursorMoved>,
    input_sender: Res<GameInputSenderResource>,
) {
    if keys.just_pressed(KeyCode::Space) {
        println!("<Space> was pressed");
        println!("Channel closed: {}", input_sender.sender.is_closed());
        let input = GameInput {
            x: 0.0,
            y: 0.0,
            pressed: true,
        };
        match input_sender.sender.send(input) {
            Ok(()) => (),
            Err(err) => eprintln!(
                "Error: Transmitting keyboardinput through channel failed: {}",
                err
            ),
        };
    }
    for cursor_event in cursor_evr.iter() {
        let input = GameInput {
            x: cursor_event.position.x,
            y: cursor_event.position.y,
            pressed: true,
        };
        match input_sender.sender.send(input) {
            Ok(()) => (),
            Err(err) => eprintln!(
                "Error: Transmitting mouse input through channel failed: {}",
                err
            ),
        };
    }
}

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
