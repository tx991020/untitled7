// give a rust tokio select example

use std::fmt::Debug;
use std::net::TcpStream;
use futures_util::SinkExt;

use tokio::select;
use tokio::signal::ctrl_c;
use tokio::sync::mpsc;

async fn run() {
    let (mut tx, mut rx) = mpsc::channel(1);

    tokio::spawn(async move {
        //tokio ticker example
        let mut ticker = tokio::time::interval(std::time::Duration::from_secs(1));

        // loop select else
        loop {
            select! {
                Some(t)=rx.recv() => {
                    println!("rx recv");
                }
               _ = ticker.tick() => {
                    println!("ticker tick");
                }

                else => {
                    println!("else");
                }

            }
        }
    });
    tokio::spawn(async move {
        //tokio ticker example
        let mut ticker = tokio::time::interval(std::time::Duration::from_secs(1));

        loop {
            tx.send("hello".to_string()).await.unwrap();
            ticker.tick().await;
        }
    });
}




#[tokio::main]
async fn main() {
    run().await;
    ctrl_c().await.unwrap();
}
