use std::{error::Error, net::SocketAddr};

use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
use tracing::{debug, info, info_span, Instrument};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    run_server().await
}

// 👇 here we use an attribute macro
#[tracing::instrument]
async fn run_server() -> Result<(), Box<dyn Error>> {
    let addr: SocketAddr = "0.0.0.0:3779".parse()?;
    info!("Listening on http://{}", addr);
    let listener = TcpListener::bind(addr).await?;
    loop {
        //                      here we use a Future extension trait 👇
        let (stream, addr) = listener.accept().instrument(info_span!("accept")).await?;
        handle_connection(stream, addr).await?;
    }
}

#[tracing::instrument(skip(stream))]
async fn handle_connection(mut stream: TcpStream, addr: SocketAddr) -> Result<(), Box<dyn Error>> {
    let req = read_http_request(&mut stream).await?;
    debug!(%req, "Got HTTP request");
    write_http_response(&mut stream).await?;

    Ok(())
}

#[tracing::instrument(skip(stream))]
async fn read_http_request(mut stream: impl AsyncRead + Unpin) -> Result<String, Box<dyn Error>> {
    let mut incoming = vec![];

    loop {
        let mut buf = vec![0u8; 1024];


        let read = stream.read(&mut buf).await?;
        incoming.extend_from_slice(&buf[..read]);

        if incoming.len() > 4 && &incoming[incoming.len() - 4..] == b"\r\n\r\n" {
            break;
        }
    }

    Ok(String::from_utf8(incoming)?)
}

#[tracing::instrument(skip(stream))]
async fn write_http_response(mut stream: impl AsyncWrite + Unpin) -> Result<(), Box<dyn Error>> {
    stream.write_all(b"HTTP/1.1 200 OK\r\n").await?;
    stream.write_all(b"\r\n").await?;
    stream.write_all(b"Hello from plaque!\n").await?;
    Ok(())
}
