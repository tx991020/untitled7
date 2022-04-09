use std::net::UdpSocket;
use anyhow::{Context, Result};

use untitled7::common::{Connection, new_server, to_socket_addr};

#[tokio::main]
async fn main() -> Result<()> {
    let addr = to_socket_addr("127.0.0.1:6000").unwrap();


    let (mut end ) = new_server(addr)?;


    let mut conn = Connection::new_for_server(&mut end).await?.context("none")?;

    while let Some(Ok( data)) = conn.next().await {

        let message = String::from_utf8_lossy(data.as_slice());
        match &message[..] {
            "ping" => {
                println!("[server] received \"ping\" sending \"pong\"");
                conn.send_raw("hello".as_bytes()).await?;

            }
            "marco" => {
                println!("[server] received \"marco\" sending \"polo\"");
                conn.send_raw("world".as_bytes()).await;
            }
            it => println!("[server] Unknown message {:?}", it),
        }
    }
    Ok(())
}