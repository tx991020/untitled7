

use std::thread::sleep;
use std::time::Duration;
use chrono::prelude::*;

#[actix_rt::main]
async fn main() {
    println!("Hello, world!");
    actix_rt::task::spawn_blocking(
        || {
            sleep(Duration::from_secs(10));
            println!("{},{}", "hha1", Local::now().timestamp());
        }
    );


    actix_rt::task::spawn_blocking(
        ||{
            sleep(Duration::from_secs(5));macintosh hd 是只读宗卷
            println!("{},{}", "hha2", Local::now().timestamp());
        }
    );

    actix_rt::signal::ctrl_c().await;
}
