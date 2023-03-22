use crate::*;

pub fn handle_keyboard(keys: Res<Input<KeyCode>>, ws_client: Res<WebsocketClient>) {
    if keys.just_pressed(KeyCode::Space) {
        println!("<Space> was pressed");
        let input = generic_protobufs::ClientInput {
            x: 0.0,
            y: 0.0,
            pressed: true,
        };
        match ws_client.send_message(input, 20, true) {
            Ok(_) => (),
            Err(NetworkError::NotConnected) => (), //eprintln!("Failed to send message to server because there is no server."),
            Err(unknown_err) => eprintln!(
                "Failed to send message for an unknown reason: {}",
                unknown_err
            ),
        };
    }
}

pub fn handle_mouse(
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    ws_client: Res<WebsocketClient>
) {
    let (camera, camera_transform) = cameras.single();
    for cursor_event in cursor_moved_events.iter() {
        if let Some(mouse_pos) = camera.viewport_to_world_2d(camera_transform, cursor_event.position) {
            let input = generic_protobufs::ClientInput {
                x: mouse_pos.x,
                y: mouse_pos.y,
                pressed: true,
            };
            match ws_client.send_message(input, 20, false) {
                Ok(_) => (),
                Err(NetworkError::NotConnected) => (), //eprintln!("Failed to send message to server because there is no server."),
                Err(unknown_err) => eprintln!(
                    "Failed to send message for an unknown reason: {}",
                    unknown_err
                ),
            };
        }
    }
}
