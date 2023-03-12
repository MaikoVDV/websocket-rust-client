use crate::*;

pub fn handle_keyboard(
    keys: Res<Input<KeyCode>>,
    ws_client: Res<WebsocketClient>,
) {
    if keys.just_pressed(KeyCode::Space) {
        println!("<Space> was pressed");
        let input = proto_all::ClientInput {
            x: 0.0,
            y: 0.0,
            pressed: true,
        };
        match ws_client.send_message(input, 1) {
            Ok(_) => (),
            Err(NetworkError::NotConnected) => (), //eprintln!("Failed to send message to server because there is no server."),
            Err(unknown_err) => eprintln!("Failed to send message for an unknown reason: {}", unknown_err)
        };
    }
}

pub fn handle_mouse(
    mut cursor_evr: EventReader<CursorMoved>,
    ws_client: Res<WebsocketClient>,
) {
    for cursor_event in cursor_evr.iter() {
        let input = proto_all::ClientInput {
            x: cursor_event.position.x,
            y: cursor_event.position.y,
            pressed: true,
        };
        match ws_client.send_message(input, 1) {
            Ok(_) => (),
            Err(NetworkError::NotConnected) => (), //eprintln!("Failed to send message to server because there is no server."),
            Err(unknown_err) => eprintln!("Failed to send message for an unknown reason: {}", unknown_err)
        };
    }
}
