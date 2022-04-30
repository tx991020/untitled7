use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Msg {
    pub room: String,
    pub username: String,
    pub timestamp: u64,
}

impl TryFrom<&str> for Msg {
    type Error = serde_json::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(value)
    }
}

impl TryFrom<&Msg> for String {
    type Error = serde_json::Error;

    fn try_from(value: &Msg) -> Result<Self, Self::Error> {
        serde_json::to_string(value)
    }
}

//
// Language: rust

use axum::{routing::get, Router, Json};
use std::net::SocketAddr;
use axum::handler::Handler;
use axum::response::IntoResponse;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index));


    // build our application with a route


    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> impl IntoResponse {
    Json(Msg {
        room: "test".to_string(),
        username: "test".to_string(),
        timestamp: 0,
    })
}






//
// fn main() -> Result<()> {
//     let msg = &Msg {
//         room: "1".into(),
//         username: "2".into(),
//         timestamp: 0,
//     };
//
//     let x: String = msg.try_into()?;
//     dbg!(&x);
//
//     let x1: Msg = x.as_str().try_into()?;
//     dbg!(x1);
//
//     Ok(())
// }
