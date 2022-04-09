use std::fs::OpenOptions;
use anyhow::Result;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::fs::File;
use futures::SinkExt;
use tokio_util::codec::{Framed, BytesCodec, FramedRead};

use bytes::Bytes;



#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {

    let my_async_read = File::open("/Users/andy/CLionProjects/untitled7/xx.txt").await?;
    let my_stream_of_bytes = Framed::new(my_async_read, BytesCodec::new());



    Ok(())
}