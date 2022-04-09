use tokio::sync::oneshot;
use tokio::time::{Duration, sleep, interval, interval_at, Instant};
use smol::Timer;


//actor 模型，grpc,sciter,channel,锁，tokio,future
#[tokio::main]
async fn main() {
    let (send, mut recv) = oneshot::channel();


    let handle =
        tokio::spawn(async move {
            sleep(Duration::from_secs(1)).await;
            send.send("shut down").unwrap();
        });


    loop {
        tokio::select! {
                _ = Timer::after(Duration::from_millis(30)) => println!("Another 100ms"),
                 _ = interval.tick() => println!("Another 20ms"),
                msg = &mut recv => {
                    println!("Got message: {}", msg.unwrap());
                    break;
                }
            }
    }
    handle.await.unwrap();
}