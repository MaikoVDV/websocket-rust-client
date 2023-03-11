use crate::*;

use futures_util::stream::SplitSink;

// Runs on a separate thread.
// Listens for changes in GameInput, and then transmits them via the websocket.
pub async fn broadcast_game_input(
    mut input_receiver: watch::Receiver<proto_all::ClientInput>,
    //mut ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    mut ws_sink: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
) {
    let input_send_interval = 20; // Sending inputs x times per second
    info!(
        "Websocket broadcaster thread created. Entering loop and sending inputs at {} tps",
        input_send_interval
    );
    let mut interval =
        tokio::time::interval(time::Duration::from_millis(1000 / input_send_interval));
    loop {
        match input_receiver.changed().await {
            Ok(()) => {}
            Err(e) => {
                info!("Failed to check if input changed. Error message: {}", e);
                break;
            }
        }
        let data = serialize_gameinput(&input_receiver.borrow());
        ws_sink.send(Message::binary(data)).await.unwrap();
        interval.tick().await;
    }
}

// Converts GameInput created by the client to a byte array which will be used in a Message::Binary websocket message.
fn serialize_gameinput(input: &proto_all::ClientInput) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    let mut writer = Writer::new(&mut out);
    writer.write_u8(1).unwrap(); // Header of 1. Indicates that this is a GameInput
    writer
        .write_message_no_len(input)
        .expect("Cannot serialize input.");
    return out;
}
