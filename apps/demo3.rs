use std::thread::sleep;
use std::time::Duration;
use actix_web::web;
use chrono::prelude::*;

async fn todo() {
    sleep(Duration::from_secs(10));
    println!("{}", "hha1");

}

async fn todo1() {
    sleep(Duration::from_secs(5));
    println!("{}", "hha2");

}

// #[actix_web::main]
// async fn main() {

//
//
//     sleep(Duration::from_secs(20));
//
// }


use actix_web::{get, App, HttpServer, Responder};

#[get("/hello1")]
async fn index() -> impl Responder {
    println!("{},{}", "hha" ,Local::now().timestamp() );
    web::block(|| {
        sleep(Duration::from_secs(10));
        println!("{},{}", "hha1" ,Local::now().timestamp() );
    });

    web::block(|| {
        sleep(Duration::from_secs(5));
        println!("{},{}", "woo",Local::now().timestamp());
    });


    // let task1 = tokio::spawn(async move {
    //     sleep(Duration::from_secs(10));
    //     println!("{},{}", "hha1" ,Local::now().timestamp() );
    // });
    // let task = tokio::spawn(async move {
    //     sleep(Duration::from_secs(5));
    //     println!("{},{}", "woo",Local::now().timestamp());
    // });

    "hello world"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8083")?
        .run()
        .await
}
