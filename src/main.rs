// Importing local modules
mod game; // Several modules that control everything to do with the game
mod networking;
mod proto; // Converts data stored in memory to messages to be sent via websocket
mod utils;

// Importing from local modules
use game::{
    game_world,
    update_internal_state::listen_for_state_changes,
    handle_input,
    components,
    components::generic::*,
};
use networking::{
    connection_manager::WebsocketClient,
    connection::*,
    connection_events::*,
    network_plugin,
    network_plugin::{AppNetworkClientMessage, NetworkMessage, SyncChannel},
    listen::listen,
    network_messages,
    serialization::proto_serialize,
    net_errors::NetworkError,
    broadcast::broadcast,
};
use proto::proto_all;
use utils::*;

// Bevy imports
use bevy::{
    prelude::*,
    render::{
        settings::{Backends, WgpuSettings},
        RenderPlugin,
    },
};

// Networking & Multithreading (tokio)
use tokio::{
    sync::mpsc,
    net::TcpStream,
    task::JoinHandle,
    runtime,
};
use dashmap::DashMap;

use tokio_tungstenite::{
    connect_async,
    tungstenite::protocol::Message,
    MaybeTlsStream,
    WebSocketStream,
};

use quick_protobuf::{BytesReader, MessageRead, Writer};
use crossbeam_channel::{Sender as Crossbeam_Sender, Receiver as Crossbeam_Receiver};

// Futures
use futures_util::{SinkExt, StreamExt, stream::{SplitSink, SplitStream}};

// Standard Library imports
use std::{
    collections::HashMap,
    net::{SocketAddr, IpAddr, Ipv4Addr},
    sync::{Arc/* , Mutex*/},
    fmt,
};
// For easily deriving the Error trait.
use thiserror;

const PORT: u16 = 8080;
const FIXED_TIMESTEP: f32 = 1.0 / 20.0;

#[tokio::main]
async fn main() {
    

    // Creating the Bevy app.
    let mut bevy_app = App::new();
    bevy_app.add_plugins(
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
                    backends: Some(get_platform_graphics_api()),
                    ..default()
                },
            }),
    );
    // Resources
    bevy_app.insert_resource(game_world::GameWorld::new());
    bevy_app.insert_resource(FixedTime::new_from_secs(FIXED_TIMESTEP));

    // Networking
    bevy_app.add_plugin(network_plugin::ClientPlugin); // Handles all networking through the websocket.
    bevy_app.add_system(network_plugin::create_new_connection);
    bevy_app.listen_for_network_message::<network_messages::GameStateUpdateMessage>();

    // State management
    bevy_app.add_system(listen_for_state_changes);

    // Input handling
    // bevy_app.add_system(handle_input::handle_keyboard.in_schedule(CoreSchedule::FixedUpdate));
    // bevy_app.add_system(handle_input::handle_mouse.in_schedule(CoreSchedule::FixedUpdate));
    bevy_app.add_system(handle_input::handle_keyboard);
    bevy_app.add_system(handle_input::handle_mouse);

    // Running the app. This method call is blocking, and won't end until the window is closed.
    bevy_app.run();
}