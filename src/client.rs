use tokio_tungstenite::connect_async;
use tokio::io::{self, AsyncBufReadExt};
use futures::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::Message;

use crate::utils::timestamp;

pub async fn run_client(addr: String) {
    let url = format!("ws://{}", addr); // ✅ Keep as String

    println!("🔌 Connecting to server at {}", url);

    let (ws_stream, _) = connect_async(&url).await.expect("Failed to connect"); // ✅ Pass as &str
    let (mut write, mut read) = ws_stream.split();

    let stdin = io::BufReader::new(io::stdin());
    let mut lines = stdin.lines();

    tokio::spawn(async move {
        while let Ok(Some(msg)) = lines.next_line().await { // ✅ Correct match pattern
            if write.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    while let Some(Ok(msg)) = read.next().await {
        if let Message::Text(text) = msg {
            println!("{} 📨 {}", timestamp(), text);
        }
    }
}
