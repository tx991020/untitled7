

use std::{env, io, io::Error};

use anyhow::Result;
use log::info;
use tokio::net::{TcpListener, TcpStream};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = env_logger::try_init();
    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:7000".to_string());

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    info!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }

    Ok(())
}

async fn accept_connection(stream: TcpStream) {
    let addr = stream.peer_addr().expect("connected streams should have a peer address");
    info!("Peer address: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    info!("New WebSocket connection: {}", addr);

    let (mut write, mut read) = ws_stream.split();



    read.forward(write).await.expect("Failed to forward message")
}

