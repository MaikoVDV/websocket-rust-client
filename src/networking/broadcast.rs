use crate::*;

pub async fn broadcast(
    mut ws_sender: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    mut constant_msg_receiver: mpsc::UnboundedReceiver<Vec<u8>>,
    mut impulse_msg_receiver: mpsc::UnboundedReceiver<Vec<u8>>,
) {
    let mut interval = tokio::time::interval(std::time::Duration::from_millis(
        1000 / (FIXED_TIMESTEP * 1000.0) as u64,
    ));

    loop {
        // Send impulse messages
        match impulse_msg_receiver.try_recv() {
            Ok(msg) => {
                // If there is an impulse message in the channel,
                // Send it to the server
                ws_sender.send(Message::binary(msg)).await.unwrap();
            }
            Err(mpsc::error::TryRecvError::Empty) => (), // No new messages.
            Err(mpsc::error::TryRecvError::Disconnected) => {
                eprintln!("The message channel used in broadcast() has closed.
                          This is probably because the broadcast task was aborted in ServerConnection::stop().");
                return;
            }
        };

        // Send constant messages
        let latest_msg = match constant_msg_receiver.try_recv() {
            Ok(msg) => msg,
            Err(mpsc::error::TryRecvError::Empty) => {
                // No new constant messages. Wait for 1 tick and then try again.
                interval.tick().await;
                continue;
            }
            Err(mpsc::error::TryRecvError::Disconnected) => {
                eprintln!("The message channel used in broadcast() has closed. This is probably because the broadcast task was aborted in ServerConnection::stop().");
                return;
            }
        };
        // Clear other messages
        loop {
            match constant_msg_receiver.try_recv() {
                Ok(_) => (),
                Err(mpsc::error::TryRecvError::Empty) => break, // No new messages.
                Err(mpsc::error::TryRecvError::Disconnected) => {
                    eprintln!("The message channel used in broadcast() has closed. This is probably because the broadcast task was aborted in ServerConnection::stop().");
                    return;
                }
            };
        }
        if latest_msg.len() <= 0 {
            continue;
        }

        // Messages have already
        ws_sender.send(Message::binary(latest_msg)).await.unwrap();

        interval.tick().await;
    }
}
