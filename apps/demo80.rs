use tokio::sync::mpsc::channel;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, SystemTime};
use chrono::Local;
use tokio::signal::ctrl_c;
use crossbeam_channel::{bounded, tick, select};
use tokio::time::interval;


#[tokio::main]
async fn main() {
    let (tx, mut rx) = channel::<i64>(20);

    let (std1, rec1) = bounded(10);



    thread::spawn(move || {
        let ticker = tick(Duration::from_secs(1));
        loop {
            select! {
                    recv(ticker) -> _ => {

                        std1.send(Local::now().timestamp());
                    }

                }
        }
    });
    //同步发channel 发到异步channel
    thread::spawn(move || {
        while let Ok(data) = rec1.recv() {
            tx.blocking_send(data);
        }
    });


    tokio::spawn(async move {
        while let Some(rx) = rx.recv().await {
            println!("3111{:?}", rx);
        }
    });

    ctrl_c().await;
}