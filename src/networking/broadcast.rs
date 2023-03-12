use crate::*;

pub async fn broadcast(
    mut ws_sender: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    mut msg_receiver: mpsc::UnboundedReceiver<Vec<u8>>,
) {
    let mut interval =
        tokio::time::interval(std::time::Duration::from_millis(1000 / (FIXED_TIMESTEP * 1000.0) as u64));

    // NOTE: CURRENTLY JUST GRABBING THE NEWEST GAMEINPUT & TRANSMITTING IT.
    // THIS MEANS THAT SOME GAMEINPUTS ARE LOST.
    // VERY BIG ISSUE; NEED TO FIX
    loop {
        let mut oldest_msg = Vec::new();
        loop {
            match msg_receiver.try_recv() {
                Ok(msg) => oldest_msg = msg,
                Err(mpsc::error::TryRecvError::Empty) => break, // No new messages.
                Err(mpsc::error::TryRecvError::Disconnected) => {
                    eprintln!("The message channel used in broadcast() has closed. This is probably because the broadcast task was aborted in ServerConnection::stop().");
                    return;
                }
            };
        };
        if oldest_msg.len() <= 0 {
            continue;
        }
        // Messages have already 
        ws_sender.send(Message::binary(oldest_msg)).await.unwrap();
        interval.tick().await;
    }
}