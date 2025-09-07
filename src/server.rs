use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
use futures::{StreamExt, SinkExt};
use std::sync::Arc;
use tokio::sync::broadcast;
use anyhow;
use tokio::sync::Mutex;

use crate::utils::timestamp;

type Tx = broadcast::Sender<String>;

pub async fn run_server(port: u16) {
    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&addr).await.expect("Can't bind to port");
    println!("🚀 Server running on ws://{}", addr);

    let (tx, _rx) = broadcast::channel(100);

    while let Ok((stream, _)) = listener.accept().await {
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, tx, &mut rx).await {
                eprintln!("Connection error: {}", e);
            }
        });
    }
}

pub async fn handle_connection(
    stream: TcpStream,
    tx: Tx,
    rx: &mut broadcast::Receiver<String>,
) -> anyhow::Result<()> {
    let ws_stream = accept_async(stream).await?;
    let (write, mut read) = ws_stream.split();

    let write = Arc::new(Mutex::new(write));
    let mut rx = rx.resubscribe();

    let write_clone = Arc::clone(&write);
    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            let mut writer = write_clone.lock().await;
            if writer.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    while let Some(msg) = read.next().await {
        let msg = msg?;
        if let Message::Text(text) = msg {
            let _ = tx.send(text.to_string());
            println!("{} 📨 {}", timestamp(), text);
        }
    }

    Ok(())
}
