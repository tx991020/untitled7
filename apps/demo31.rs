use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

use futures::FutureExt;
use std::env;
use std::error::Error;



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listen_addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "0.0.0.0:6000".to_string());


    println!("Listening on: {}", listen_addr);


    let listener = TcpListener::bind(listen_addr).await?;

    while let Ok((mut inbound, addr)) = listener.accept().await {
        println!("11111{:?}",addr);
        let bytes1 = bytes::Bytes::from("hahhaa");
        inbound.write_all(bytes1.as_ref()).await;
        // let transfer = transfer(inbound, addr).map(|r| {
        //     if let Err(e) = r {
        //         println!("Failed to transfer; error={}", e);
        //     }
        // });

        // tokio::spawn(transfer);
    }

    Ok(())
}