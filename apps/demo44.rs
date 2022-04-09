

// use std::net::UdpSocket;
// use std::thread::sleep;
// use std::time::Duration;
//
// use anyhow::{Context, Result};
// use untitled7::common::{Connection, new_client, to_socket_addr};
// use bytes::Bytes;
//
// #[tokio::main]
// async fn main() -> Result<()> {
//
//     let peer = to_socket_addr("127.0.0.1:6000").unwrap();
//
//
//     let mut conn = new_client(peer).await?;
//     conn.send_raw("ping".as_ref()).await;
//
//     while let Some(data) = conn.next().await{
//         match data {
//             Ok(t) => {
//                 println!("xxxxxx {:?}", String::from_utf8_lossy(t.as_slice()));
//             }
//             Err(e) => {
//                 println!("err {:?}", e);
//             }
//         }
//
//
//     }
//     println!("{}",333 );
//
//     Ok(())
// }
fn main() {

}