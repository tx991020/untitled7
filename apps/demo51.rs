use std::time::Duration;
use tokio::time::sleep;
use async_static::async_static;

async fn get_num() -> i32 {
    println!("hello world");
    sleep(Duration::from_millis(100)).await;
    123
}

async_static! {
    static ref FOO:i32 = get_num().await;
}


#[tokio::main]
async fn main() {
    let n = FOO.await;
    println!("The result of the first call: {}", n);

    // The second call, nothing print
    let n = FOO.await;
    println!("The result of the second call: {}", n);
}


#[tokio::test]
async fn test() {
    // The first call, print hello world
    let n = FOO.await;
    println!("The result of the first call: {}", n);

    // The second call, nothing print
    let n = FOO.await;
    println!("The result of the second call: {}", n);
}