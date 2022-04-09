use tokio::{fs, io};
use tokio::net::{UnixListener, UnixStream};
use futures::StreamExt;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()>{
    static PATH: &'static str = "/Users/andy/CLionProjects/untitled7/yyy.sock";

    let listener = match UnixListener::bind(PATH) {
        Ok(m) => m,
        Err(_) => {
            fs::remove_file(PATH).await.unwrap();
            UnixListener::bind(PATH).unwrap()
        }
    };


    // loop {
    //     let (socket, _) = listener.accept().await?;
    //     echo(socket).await?;
    // }

    loop {
        let (mut socket, _) = listener.accept().await?;
        let (mut recv, mut send) = socket.split();

        io::copy(&mut recv,&mut send).await?;

    }
    Ok(())
}

// async fn echo(socket: UnixStream) -> io::Result<()> {
//     let (mut recv, mut send) = io::split(socket);
//     io::copy(&mut recv, &mut send).await?;
//     Ok(())
// }