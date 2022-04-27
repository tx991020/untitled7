use anyhow::{anyhow, bail, Context, Result};
use std::net::{ToSocketAddrs, UdpSocket};
use std::{fs, io, net::SocketAddr, sync::Arc};
use tokio::sync::mpsc;

use futures::stream::StreamExt;
use protobuf::Message;
use quinn::Connection as QConnection;
use quinn::{
    Certificate, CertificateChain, ClientConfig, ClientConfigBuilder, Endpoint, Incoming,
    NewConnection, PrivateKey, ReadError, SendStream, ServerConfig, ServerConfigBuilder,
    TransportConfig,
};

const SERVER_NAME: &str = "localhost";

type Value = Result<Vec<u8>>;
type Sender = mpsc::UnboundedSender<Value>;
type Receiver = mpsc::UnboundedReceiver<Value>;

pub fn configure_server() -> Result<(ServerConfig)> {
    let dirs = directories::UserDirs::new().unwrap();
    let path = dirs.home_dir();
    let cert_path = path.join("cert.der");
    let key_path = path.join("key.der");
    println!("cert path{:?}", &cert_path);
    let (cert, key) = match fs::read(&cert_path).and_then(|x| Ok((x, fs::read(&key_path)?))) {
        Ok(x) => x,
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
            println!("generating self-signed certificate");
            let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
            let key = cert.serialize_private_key_der();
            let cert = cert.serialize_der().unwrap();
            fs::create_dir_all(&path).context("failed to create certificate directory")?;
            fs::write(&cert_path, &cert).context("failed to write certificate")?;
            fs::write(&key_path, &key).context("failed to write private key")?;
            (cert, key)
        }
        Err(e) => {
            bail!("failed to read certificate: {}", e);
        }
    };

    let key = quinn::PrivateKey::from_der(&key)?;
    let cert = quinn::Certificate::from_der(&cert)?;

    let mut transport_config = TransportConfig::default();

    let mut server_config = ServerConfig::default();
    server_config.transport = Arc::new(transport_config);
    let mut cfg_builder = ServerConfigBuilder::new(server_config);
    cfg_builder.certificate(CertificateChain::from_certs(vec![cert]), key)?;

    Ok((cfg_builder.build()))
}

fn configure_client() -> Result<ClientConfig> {
    let dirs = directories::UserDirs::new().unwrap();
    let path = dirs.home_dir();
    let cert_path = path.join("cert.der");
    let cert = match fs::read(&cert_path) {
        Ok(x) => x,
        Err(e) => {
            bail!("failed to read certificate: {}", e);
        }
    };
    let mut endpoint = quinn::Endpoint::builder();
    let mut client_config = quinn::ClientConfigBuilder::default();
    client_config
        .add_certificate_authority(quinn::Certificate::from_der(&cert)?)
        .unwrap();

    Ok(client_config.build())
}

pub fn new_server(addr: SocketAddr) -> Result<(Incoming)> {
    let (mut server_config) = configure_server()?;

    let mut endpoint_builder = Endpoint::builder();
    endpoint_builder.listen(server_config);
    let (_endpoint, incoming) = endpoint_builder.bind(&addr)?;
    Ok((incoming))
}

// pub async fn new_client( peer:SocketAddr) -> Result<Connection> {
//     let client_cfg = configure_client()?;
//
//     let mut endpoint_builder = Endpoint::builder();
//     endpoint_builder.default_client_config(client_cfg);
//
//     let (endpoint, _incoming) = endpoint_builder.bind(&"[::]:0".parse().unwrap())?;
//     let new_conn = endpoint.connect(&peer, SERVER_NAME)?.await?;
//     Connection::new_for_client(new_conn.connection).await
// }

pub struct Connection {
    conn: QConnection,
    tx: SendStream,
    rx: Receiver,
}

async fn handle_request(mut rx: quinn::RecvStream, tx: Sender) -> Result<()> {
    let mut buffer: Vec<u8> = Vec::with_capacity(1024);
    loop {
        match rx.read(&mut buffer).await {
            Ok(Some(n)) => {
                tx.send(Ok(buffer[0..n].to_owned()));
            }
            Ok(None) => {
                // println!("[server] no data received");
            }
            Err(ReadError::ConnectionClosed(_)) => {
                println!("213 connection closed");
                break;
            }
            Err(e) => {
                println!("an error occurred: {:?}", e);
                break;
            }
        }
        buffer.clear();
    }

    Ok(())
}

async fn handle_request1(
    (mut send, mut recv): (quinn::SendStream, quinn::RecvStream),
    tx: Sender,
) -> Result<()> {
    let mut buffer: Vec<u8> = vec![0; 1024];
    loop {
        match recv.read(&mut buffer).await {
            Ok(Some(n)) => {
                tx.send(Ok(buffer[0..n].to_owned()));
            }
            Ok(None) => {
                // println!("[server] no data received");
            }
            Err(ReadError::ConnectionClosed(_)) => {
                println!("213 connection closed");
                break;
            }
            Err(e) => {
                println!("an error occurred: {:?}", e);
                break;
            }
        }
        buffer.clear();
    }

    send.write_all("qqqq".as_bytes()).await?;
    send.write_all("wwww".as_bytes()).await?;
    send.finish().await?;

    Ok(())
}

pub fn to_socket_addr(host: &str) -> Result<SocketAddr> {
    let addrs: Vec<SocketAddr> = host.to_socket_addrs()?.collect();
    if addrs.is_empty() {
        bail!("Failed to solve {}", host);
    }
    Ok(addrs[0])
}
