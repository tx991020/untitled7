use anyhow::Result;
use lazy_static::lazy_static;
use reqwest::{header, Client as HttpClient};
use serde_json::Value;
use std::collections::HashMap;


lazy_static! {
    static ref CLIENT: HttpClient = HttpClient::new();
}

#[tokio::main]
async fn main() -> Result<()> {
    let resp = CLIENT
        .get("https://jsonplaceholder.typicode.com/users")
        .send()
        .await?
        .json::<Value>()
        .await?;
    println!("{:#?}", resp);

    //     let res = CLIENT
    //         .post("http://127.0.0.1:8200/postTest")
    //         .body(r#"
    // {
    //   "Id": 1,
    //    "Name":"zhang"
    // }
    // "#
    //         )
    //         .send().await?
    //         .text().await?;
    //     println!("{}", res);

    Ok(())
}
