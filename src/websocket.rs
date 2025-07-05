use crate::database::DbConnector;
use crate::models::Traffic;
use futures_util::{SinkExt, StreamExt};
use std::time::Duration;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;

pub async fn init_websocket() {
    let addr = "0.0.0.0:9738";
    let listener = TcpListener::bind(&addr)
        .await
        .expect("Error");

    loop {
        let (stream, _addr) = listener.accept().await.expect("Error");
        tokio::spawn(handle_connection(stream));
    }
}

async fn handle_connection(stream: tokio::net::TcpStream) {
    let mut db = DbConnector::new();
    let ws_stream = accept_async(stream).await.expect("Error");

    let (mut write, _read) = ws_stream.split();
    loop {
        let all_traffic = db.get_traffic().await;
        send_traffic_vector(&mut write, all_traffic).await;
        tokio::time::sleep(Duration::from_secs(1)).await
    }
}

use futures_util::Sink;

async fn send_traffic_vector<S>(write: &mut S, data: Vec<Traffic>)
where
    S: Sink<Message> + Unpin,
    S::Error: std::fmt::Debug,
{
    match serde_json::to_string(&data) {
        Ok(json) => {
            let msg = Message::Text(json);
            if let Err(e) = write.send(msg).await {
                eprintln!("{:?}", e);
            }
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}
