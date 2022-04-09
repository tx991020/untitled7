
use tokio::fs;
use anyhow::Result;
#[tokio::main]
async fn main() -> Result<()> {

    fs::write("/Users/andy/CLionProjects/untitled7/apps/foo.txt", b"Hello world!").await?;
    Ok(())
}