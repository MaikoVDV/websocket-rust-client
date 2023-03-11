use crate::*;

use new_networking::listen::send_event_for_message;

#[derive(Default, Copy, Clone, Debug)]
/// The plugin to add to your bevy [`AppBuilder`](bevy::prelude::AppBuilder) when you want
/// to instantiate a client
pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        
        app.insert_resource(WebsocketClient::new());
        app.add_event::<ConnectionEvent>();
        app.add_system(handle_connection_event.in_base_set(CoreSet::PreUpdate));
        //app.add_system(listen_for_client_message::<>)
        // self.add_system_to_stage(CoreStage::PreUpdate, register_client_message::<T>.system())
        // app.add_event::<ClientNetworkEvent>();
        // app.init_resource::<NetworkSettings>();
        // app.add_system_to_stage(
        //     CoreStage::PreUpdate,
        //     client::send_client_network_events.system(),
        // );
        // app.add_system_to_stage(
        //     CoreStage::PreUpdate,
        //     client::handle_connection_event.system(),
        // );
    }
}

pub fn create_new_connection(
    mut ws_client: ResMut<WebsocketClient>,
    mouse_buttons: Res<Input<MouseButton>>,
) {
    if mouse_buttons.just_pressed(MouseButton::Left) {
        //let socket_address = format!("ws://127.0.0.1:{}", PORT);
        let socket_address = SocketAddr::new("127.0.0.1".parse().unwrap(), PORT);
        ws_client.connect(socket_address);
    }
}

// Used to pass messages synchronously. Based on std::sync::mpsc.
pub struct SyncChannel<T> {
    pub(crate) sender: Crossbeam_Sender<T>,
    pub(crate) receiver: Crossbeam_Receiver<T>,
}

impl<T> SyncChannel<T> {
    pub fn new() -> Self {
        let (sender, receiver) = crossbeam_channel::unbounded();

        SyncChannel { sender, receiver }
    }
}

/// A utility trait on the Bevy app to easily register messages to listen to by their header
pub trait AppNetworkClientMessage {
    fn listen_for_network_message<T: NetworkMessage+'static>(&mut self) -> &mut Self;
}

impl AppNetworkClientMessage for App {
    fn listen_for_network_message<T: NetworkMessage+'static>(&mut self) -> &mut Self {
        let ws_client = self.world.get_resource::<WebsocketClient>().expect("Could not find `NetworkClient`. Be sure to include the `ClientPlugin` before listening for client messages.");

        println!("Registered a new ClientMessage: {}", T::HEADER);

        assert!(
            !ws_client.state_updates.contains_key(&T::HEADER),
            "Duplicate registration of ClientMessage: {}",
            T::HEADER
        );
        ws_client.state_updates.insert(T::HEADER.to_owned(), Vec::new());

        self.add_event::<NetworkData>();
        self.add_system(send_event_for_message::<T>)
    }
}

pub trait NetworkMessage {
    const HEADER: &'static u8;
}