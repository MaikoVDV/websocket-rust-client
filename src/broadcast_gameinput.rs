use crate::*;

use tokio::net::TcpStream;
use tokio::task::unconstrained;
use tokio_tungstenite::{WebSocketStream, MaybeTlsStream};

use futures_util::{FutureExt, stream::SplitSink};

// Runs on a separate thread.
// Listens for changes in GameInput, and then transmits them via the websocket.
pub async fn broadcast_game_input(
    mut input_receiver: UnboundedReceiver<GameInput>,
    //mut ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    mut ws_sink: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
) {
    println!("Websocket broadcaster thread created. Entering loop.");
    loop {
        while let Some(is_gameinput) = unconstrained(input_receiver.recv()).now_or_never() {
            if let Some(gameinput) = is_gameinput {
                //println!("Transmitting GameInput data via websocket.");
                ws_sink.send(Message::binary(serialize_gameinput(gameinput))).await.unwrap();
            }
        }
    }
}

// Converts GameInput created by the client to a byte array which will be used in a Message::Binary websocket message.
fn serialize_gameinput(input: GameInput) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    let mut writer = Writer::new(&mut out);
    writer.write_u8(0).unwrap(); // Header, in case we wanna use different headers in the future
    writer.write_message_no_len(&input)
        .expect("Cannot serialize input.");
    return out;
}