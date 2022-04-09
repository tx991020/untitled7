

use tokio::net::UdpSocket;
use std::io;
use tokio::select;

#[tokio::main]
async fn main() -> io::Result<()> {

    let socket = UdpSocket::bind("127.0.0.1:14000").await?;
    // let socket = UdpSocket::bind("127.0.0.1:8081").await?;
    println!("Listening on {}", socket.local_addr()?);
    //
    let msg = "hello world";
    // println!("<- {}", msg);
    socket.send_to(msg.as_bytes(), "127.0.0.1:12000").await?;

    loop {
        select! {
            Some(Ok((bytes, _))) = socket.next() => {
                println!("{}",String::from_utf8_lossy(&bytes))
            }
        }
    }


    Ok(())
}