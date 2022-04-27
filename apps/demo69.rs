use std::collections::HashMap;
use actix::fut::result;
use untitled7::items::{ClientRequest, Shirt};
use untitled7::items::shirt::Size;
use untitled7::items::Payload;
use prost::Message;
fn main() {
    let pid = std::process::id();
    println!("scanner pid : {:?}", pid);
    let shirt = Shirt { color: "111".to_string(), size: Size::Large.into() };
    let map = HashMap::from([
        ("a".to_string(), "1".to_string()),
    ]);
    let payload = Payload {
        fields: map,
    };
    let mut tt = ClientRequest { num_resp: 1, data: Option::from(payload) };

    let mut buf = vec![];
    tt.encode(&mut buf).unwrap();


    let decoded: ClientRequest = Message::decode(&buf[..]).unwrap();
    dbg!(decoded);



}