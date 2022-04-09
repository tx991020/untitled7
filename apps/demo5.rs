

use std::collections::HashMap;
use std::io;

use structopt::StructOpt;
use tokio::io::copy;
use tokio::net::{TcpListener, TcpStream};
use tokio::task::JoinHandle;

#[derive(Eq, PartialEq, Hash,Debug)]
enum TaskType {
    WriteTask,
    ReadTask,
}

async fn process_conn(local: TcpStream, remote: TcpStream) {
    let (mut local_reader, mut local_writer) = local.into_split();
    let (mut remote_reader, mut remote_writer) = remote.into_split();

    let mut tasks_map: HashMap<TaskType, JoinHandle<_>> = HashMap::new();

    let write_task = tokio::spawn(async move {
        copy(&mut local_reader, &mut remote_writer).await
    });
    tasks_map.insert(TaskType::WriteTask, write_task);

    let read_task = tokio::spawn(async move {
        copy(&mut remote_reader, &mut local_writer).await
    });

}

/// A simple tcp forwarding tool
#[derive(StructOpt, Debug)]
#[structopt(name = "tcpforward")]
struct Options {
    /// local ip
    #[structopt(long)]
    local_ip: String,

    /// local port
    #[structopt(long)]
    local_port: u16,

    /// remote ip
    #[structopt(long)]
    remote_ip: String,

    /// remote port
    #[structopt(long)]
    remote_port: u16,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("service is starting ...");
    let options:Options = Options{
        local_ip: "127.0.0.1".to_string(),
        local_port: 7000,
        remote_ip: "127.0.0.1".to_string(),
        remote_port: 9000
    };



    //本地服务器
    let listener = TcpListener::bind(
        format!("{}:{}", options.local_ip, options.local_port)
    ).await?;

    loop {
        let (local, peer_addr) = listener.accept().await?;
        println!("a new connection {:?} is coming!", peer_addr);

        //连接远程
        let remote = match TcpStream::connect(
            format!("{}:{}", options.remote_ip, options.remote_port)
        ).await {
            Ok(s) => s,
            Err(e) => {
                println!("connect to remote error: {:?}", e.to_string());
                continue;
            }
        };
        tokio::spawn(async move {
            process_conn(local, remote).await;
        });
    }
}