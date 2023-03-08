use crate::*;

use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

#[derive(Resource, Debug)]
pub struct WebsocketStream {
    pub stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

// Creates the websocket.
pub async fn init_websocket_connection() -> WebSocketStream<MaybeTlsStream<TcpStream>> {
    let addr = format!("ws://127.0.0.1:{}", PORT);
    let url = url::Url::parse(&addr).unwrap();
    println!("Starting tokio with url {}", url);

    let (ws_stream, _) = connect_async(url)
        .await
        .expect("Failed to connect to the server");
    println!("WebSocket handshake has been successfully completed");
    return ws_stream;

    // let ws_to_stdout = {
    //     read.for_each(|message| async {
    //         let data = message.unwrap().into_data();
    //         tokio::io::stdout().write_all(&data).await.unwrap();
    //     })
    // };
}

