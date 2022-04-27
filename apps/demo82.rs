use futures::{stream, StreamExt};
use rand::{thread_rng, Rng};
use std::time::Duration;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    stream::iter(0..20)
        .for_each_concurrent(4, |number| async move {
            let mut rng = thread_rng();
            let sleep_ms: u64 = rng.gen_range(0,20);
            tokio::time::sleep(Duration::from_millis(sleep_ms)).await;
            println!("{}", number);
        })
        .await;
}