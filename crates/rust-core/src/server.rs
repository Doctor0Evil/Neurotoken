use crate::backend::{NeuromorphicBackend, BackendError, CommandResult};
use crate::command::Command;
use anyhow::Result;
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WsMessage {
    Command(Command),
    Result(CommandResult),
    Error { message: String },
}

pub async fn run_server(addr: &str) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;
    println!("rust-core websocket server listening on {}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(handle_conn(stream));
    }
}

async fn handle_conn(stream: tokio::net::TcpStream) {
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            eprintln!("ws accept error: {}", e);
            return;
        }
    };

    let (mut write, mut read) = ws_stream.split();
    let mut backend = NeuromorphicBackend::new();

    while let Some(msg) = read.next().await {
        match msg {
            Ok(m) if m.is_text() => {
                let text = m.into_text().unwrap();
                let parsed: Result<WsMessage, _> = serde_json::from_str(&text);
                match parsed {
                    Ok(WsMessage::Command(cmd)) => {
                        let reply = match backend.handle(cmd) {
                            Ok(res) => WsMessage::Result(res),
                            Err(e) => WsMessage::Error {
                                message: backend_error_to_string(e),
                            },
                        };
                        let _ = write.send(tokio_tungstenite::tungstenite::Message::Text(
                            serde_json::to_string(&reply).unwrap(),
                        )).await;
                    }
                    Ok(_) => {}
                    Err(e) => {
                        let _ = write.send(tokio_tungstenite::tungstenite::Message::Text(
                            serde_json::to_string(&WsMessage::Error {
                                message: format!("invalid message: {}", e),
                            }).unwrap(),
                        )).await;
                    }
                }
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("ws error: {}", e);
                break;
            }
        }
    }
}

fn backend_error_to_string(e: BackendError) -> String {
    e.to_string()
}
