
#[macro_use]
extern crate tracing;


fn main() {
    let file_appender = tracing_appender::rolling::hourly("/Users/andy/CLionProjects/untitled7", "prefix.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let _collector = tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .init();

    info!("{}","haha");

}