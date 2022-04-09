use std::io;
use anyhow::Result;
use tokio;
use async_process::Command;

#[tokio::main]
async fn main() ->Result<()>{
    let mut cmd = Command::new("ping www.baidu.com");
    cmd.stdin(io::stdin());
    cmd.stdout(io::stdout());
    cmd.stderr(io::stderr());
    let output = cmd.output().await?;
    let stdout = output.stdout;
    let stderr = output.stderr;
    let status = output.status;


    let cow = String::from_utf8_lossy(stdout.as_slice());
    println!("{}",cow );

    Ok(())

}