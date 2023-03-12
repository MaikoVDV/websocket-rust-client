use crate::*;

pub async fn broadcast(
    mut ws_sender: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    mut msg_receiver: mpsc::UnboundedReceiver<Vec<u8>>,
) {
    loop {
        let msg = match msg_receiver.try_recv() {
            Ok(msg) => msg,
            Err(mpsc::error::TryRecvError::Empty) => continue, // No new messages.
            Err(e) => {
                info!("Failed to check if input changed. Error message: {}", e);
                continue;
            }
        };
        // Messages have already 
        ws_sender.send(Message::binary(msg)).await.unwrap();
    }
}