// Importing local modules
mod gameplay; // Several modules that control everything to do with the game
mod networking;
mod proto; // Converts data stored in memory to messages to be sent via websocket

// Importing from local modules
use gameplay::*; // Exposing sub-modules
use networking::{
    broadcast_gameinput::broadcast_game_input, // Creates thread that checks for new GameInputs (with a tokio watch channel) and then sends that to the server
    connection::init_websocket_connection,     // Opens & manages the websocket
    receive_state::get_game_state, // Creates thread that listens for state changes over the websocket // Creates thread that broadcasts new GameInputs to the server via the websocket
};
use proto::proto_all;

// Bevy imports
use bevy::{
    prelude::*,
    render::{
        settings::{Backends, WgpuSettings},
        RenderPlugin,
    },
};

// Networking & Multithreading (tokio)
use tokio::sync::{mpsc, watch};

use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

use quick_protobuf::Writer;

// Futures
use futures_util::{SinkExt, StreamExt};

// Standard Library imports
use std::{collections::HashMap, time};

const PORT: &str = "8080";
//const TIMESTEP: f32 = 1.0 / 60.0; // 60tps server

#[tokio::main]
async fn main() {
    // Creating a websocket for communication with the server.
    let ws_stream = init_websocket_connection().await;
    let (ws_sender, ws_receiver) = ws_stream.split();

    // Creating a thread to handle sending GameInputs to the server.
    // Also, creating an mpsc channel to send the inputs from the Bevy app to the thread.
    println!("Creating channel for gameinput communication between threads.");
    let (input_sender, input_receiver) =
        watch::channel::<proto_all::ClientInput>(proto_all::ClientInput::default());
    tokio::spawn(broadcast_game_input(input_receiver, ws_sender));

    // Creating a thread to handle receiving new gamestates from the server.
    // Also, creating an mpsc channel to send the state from the thread to the main thread, and to
    // the Bevy app.
    println!("Creating channel for state communication between threads.");
    let (state_sender, state_receiver) = mpsc::unbounded_channel::<proto_all::GameState>();
    let (state_update_sender, state_update_receiver) =
        mpsc::unbounded_channel::<proto_all::GameStateUpdate>();
    tokio::spawn(get_game_state(
        state_sender,
        state_update_sender,
        ws_receiver,
    ));

    // Setup native Graphics API for each platform.
    let platform_api = if cfg!(target_os = "windows") {
        Backends::DX12
    } else if cfg!(target_os = "macos") {
        Backends::METAL
    } else if cfg!(target_os = "linux") {
        Backends::GL
    } else {
        panic!("Unsupported platform!");
    };

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
                        backends: Some(platform_api),
                        ..default()
                    },
                }),
        )
        .insert_resource(TokioChannels {
            client_input_sender: input_sender,
            game_state_receiver: state_receiver,
            game_state_update_receiver: state_update_receiver,
        })
        .add_system(handle_input)
        .run();
}

fn handle_input(
    keys: Res<Input<KeyCode>>,
    mut cursor_evr: EventReader<CursorMoved>,
    tokio_channels: Res<TokioChannels>,
) {
    if keys.just_pressed(KeyCode::Space) {
        println!("<Space> was pressed");
        println!(
            "Channel closed: {}",
            tokio_channels.client_input_sender.is_closed()
        );
        let input = proto_all::ClientInput {
            x: 0.0,
            y: 0.0,
            pressed: true,
        };
        match tokio_channels.client_input_sender.send(input) {
            Ok(()) => (),
            Err(err) => eprintln!(
                "Error: Transmitting keyboardinput through channel failed: {}",
                err
            ),
        };
    }
    for cursor_event in cursor_evr.iter() {
        let input = proto_all::ClientInput {
            x: cursor_event.position.x,
            y: cursor_event.position.y,
            pressed: true,
        };
        match tokio_channels.client_input_sender.send(input) {
            Ok(()) => (),
            Err(err) => eprintln!(
                "Error: Transmitting mouse input through channel failed: {}",
                err
            ),
        };
    }
}

#[derive(Resource, Debug)]
pub struct TokioChannels {
    pub client_input_sender: watch::Sender<proto_all::ClientInput>,
    pub game_state_receiver: mpsc::UnboundedReceiver<proto::proto_all::GameState>,
    pub game_state_update_receiver: mpsc::UnboundedReceiver<proto_all::GameStateUpdate>,
}
//#[derive(Resource, Debug)]
//pub struct GameInputSenderResource {
//    pub sender: watch::Sender<ClientInput>,
//}
//
//#[derive(Resource, Debug)]
//pub struct StateReceiverResource {
//    pub receiver: mpsc::UnboundedReceiver<proto::proto_all::GameState>,
//}
