// Importing local modules
mod game; // Several modules that control everything to do with the game
mod networking;
mod proto; // Converts data stored in memory to messages to be sent via websocket
mod utils;

// Importing from local modules
use game::{
    components,
    components::generic::*,
    game_world, handle_input,
    update_internal_state::listen_for_network_events,
};
use networking::{
    broadcast::broadcast,
    connection::*,
    connection_events::*,
    connection_manager::WebsocketClient,
    listen::listen,
    net_errors::NetworkError,
    network_messages, network_plugin,
    network_plugin::{AppNetworkClientMessage, NetworkMessage, SyncChannel},
    serialization::proto_serialize,
};
use proto::*;
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
use dashmap::DashMap;
use tokio::{net::TcpStream, runtime, sync::mpsc, task::JoinHandle};

use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};

use crossbeam_channel::{Receiver as Crossbeam_Receiver, Sender as Crossbeam_Sender};
use quick_protobuf::{BytesReader, MessageRead, Writer};

// Futures
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};

// Standard Library imports
use std::{
    collections::HashMap,
    fmt,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
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
    bevy_app.add_startup_system(setup);

    // Resources
    bevy_app.insert_resource(game_world::GameWorld::new());
    bevy_app.insert_resource(FixedTime::new_from_secs(FIXED_TIMESTEP));

    // Networking
    bevy_app.add_plugin(network_plugin::ClientPlugin); // Handles all networking through the websocket.
    bevy_app.add_system(network_plugin::create_new_connection);
    bevy_app.listen_for_network_message::<network_messages::GameStateUpdateMessage>();
    bevy_app.listen_for_network_message::<network_messages::InitialStateMessage>();

    // State management
    bevy_app.add_system(listen_for_network_events);
    // bevy_app.add_system(listen_for_state_updates);
    // bevy_app.add_system(listen_for_initial_state);

    // Input handling
    bevy_app.add_system(handle_input::handle_keyboard);
    bevy_app.add_system(handle_input::handle_mouse);

    // Debugging
    bevy_app.add_system(utils::display_players);

    // Running the app. This method call is blocking, and won't end until the window is closed.
    bevy_app.run();
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        TextBundle::from_section(
            "Player display",
            TextStyle {
                font_size: 13.0,
                font: asset_server.load("fonts/PixelEmulator-xq08.ttf"),
                color: Color::WHITE,
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::Left)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..default()
            },
            ..default()
        }),
        components::debug::PlayerLogText,
    ));
}