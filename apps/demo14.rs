
#[macro_use]
extern crate tracing;



use smol::channel;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tokio::sync::channel::{bounded};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};


fn main() {

    let file_appender = tracing_appender::rolling::hourly("/Users/andy/CLionProjects/untitled7", "prefix");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let stdout = std::io::stdout.with_max_level(tracing::Level::INFO);

    let _collector = tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .init();


    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel(100);

    let (tx1, rx1) = mpsc::channel::<String>(100);


    info!("{}","wowow");

}