
use lazy_static::lazy_static;
use std::time::Duration;
use tokio::time::sleep;
use async_once::AsyncOnce;
lazy_static!{
       static ref FOO : AsyncOnce<u32> = AsyncOnce::new(async{
           1
       });
     static ref BAR : AsyncOnce<u32> = AsyncOnce::new(async{
            println!("hello world");
            sleep(Duration::from_millis(100)).await;
            123
       });
   }

#[tokio::main]
async fn main() {
    let x = FOO.get().await;
    println!("{:?}",x);
    let x = FOO.get().await;
    println!("22{:?}",x);

    let x = BAR.get().await;
    println!("{:?}",x);
    let x = BAR.get().await;
    println!("x{:?}",x);
}