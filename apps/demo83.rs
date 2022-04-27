use tokio_stream::StreamExt;
use tokio::sync::mpsc::channel;
use tokio_stream::wrappers::ReceiverStream;
#[tokio::main]
async fn main() {
    let (tx, rx) = channel(10);


    for i in 0..10 {
        tx.send(i)
    }
    drop(tx);


    let stream = ReceiverStream::new(rx);
    let v: Vec<i32> = stream.collect().await;
    println!("{:?}", v);

}