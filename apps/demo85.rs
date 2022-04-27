use std::thread;
use tokio::signal::ctrl_c;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(20);


    for i in 0..10 {
        tx.send(i).await;
    }
   rx.close();



    thread::spawn(move || {

    });


    while let Some(msg) = rx.recv().await {
        println!("got {}", msg);
    }
    ctrl_c().await;

    // Channel closed and no messages are lost.
}