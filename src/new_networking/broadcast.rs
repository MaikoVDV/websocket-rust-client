use crate::*;

impl WebsocketClient {
    pub fn broadcast(tokio_channels: ResMut<TokioChannels>) {
        // let pool = IoTaskPool::get();
        // let cc = comm_channels.tx.clone();
        // let task = pool.spawn(async move {
        //     let api_response_text = reqwest::get("http://localhost:8081/test")
        //         .await
        //         .unwrap()
        //         .text()
        //         .await
        //         .unwrap();
        //     cc.try_send(api_response_text);
        // });
        let task_pool = IoTaskPool::get();
    }
}
