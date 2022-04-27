use s2n_quic::Server;
use std::error::Error;


/// NOTE: this certificate is to be used for demonstration purposes only!
pub static CERT_PEM: &str = include_str!("/Users/andy/CLionProjects/untitled7/cert.pem");
/// NOTE: this certificate is to be used for demonstration purposes only!
pub static KEY_PEM: &str = include_str!("/Users/andy/CLionProjects/untitled7/key.pem");

#[tokio::main]
async fn main() ->Result<(), Box<dyn Error>>  {
    let mut server = Server::builder()
        .with_tls((CERT_PEM, KEY_PEM))?
        .with_io("127.0.0.1:4433")?
        .start()?;

    while let Some(mut connection) = server.accept().await {
        // spawn a new task for the connection
        tokio::spawn(async move {
            println!("Connection accepted from {:?}", connection.remote_addr());


            while let Ok(Some(mut stream)) = connection.accept_bidirectional_stream().await {
                // spawn a new task for the stream
                tokio::spawn(async move {
                    println!("Stream opened from {:?}", stream.connection().remote_addr());

                    // echo any data back to the stream
                    while let Ok(Some(data)) = stream.receive().await {
                        stream.send(data).await.expect("stream should be open");
                    }
                });
            }
        });
    }

    Ok(())
}