use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs, lookup_host, TcpSocket};

use futures::FutureExt;
use std::env;
use std::error::Error;
use std::net::SocketAddr;
use anyhow::{bail,Result};


fn new_socket(addr: std::net::SocketAddr, reuse: bool) -> Result<TcpSocket> {
    let socket = match addr {
        std::net::SocketAddr::V4(..) => TcpSocket::new_v4()?,
        std::net::SocketAddr::V6(..) => TcpSocket::new_v6()?,
    };
    if reuse {
        // windows has no reuse_port, but it's reuse_address
        // almost equals to unix's reuse_port + reuse_address,
        // though may introduce nondeterministic bahavior
        #[cfg(unix)]
            socket.set_reuseport(true)?;
        socket.set_reuseaddr(true)?;
    }
    socket.bind(addr)?;
    Ok(socket)
}
const DEFAULT_BACKLOG: u32 = 128;

pub async fn new_listener<T: ToSocketAddrs>(addr: T, reuse: bool) -> Result<TcpListener> {
    if !reuse {
        Ok(TcpListener::bind(addr).await?)
    } else {
        for addr in lookup_host(&addr).await? {
            log::info!("{:?}, {}:{}:{}:{}",format!("connection new_listener tcp addr {}",&addr),module_path!(),file!(),line!(),column!());
            let socket = new_socket(addr, true)?;
            return Ok(socket.listen(DEFAULT_BACKLOG)?);
        }
        bail!("could not resolve to any address");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listen_addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "0.0.0.0:6000".to_string());


    println!("Listening on: {}", listen_addr);

    let listener = new_listener(format!("0.0.0.0:{}",6000), true).await?;
    let addr = listener.local_addr()?;
    println!("333333{}", addr);

    while let Ok((inbound, addr)) = listener.accept().await {
        println!("11111{:?}",&addr.ip());
        let addr =   format!("{}:13389",addr.ip());
        let transfer = transfer(inbound, addr).map(|r| {
            if let Err(e) = r {
                println!("Failed to transfer; error={}", e);
            }
        });

        tokio::spawn(transfer);
    }

    Ok(())
}

async fn transfer(mut inbound: TcpStream, proxy_addr: String) -> Result<(), Box<dyn Error>> {
    let mut outbound = TcpStream::connect(proxy_addr).await?;

    let (mut ri, mut wi) = inbound.split();
    let (mut ro, mut wo) = outbound.split();

    let client_to_server = async {
        io::copy(&mut ri, &mut wo).await?;
        wo.shutdown().await
    };

    let server_to_client = async {
        io::copy(&mut ro, &mut wi).await?;
        wi.shutdown().await
    };

    tokio::try_join!(client_to_server, server_to_client)?;

    Ok(())
}