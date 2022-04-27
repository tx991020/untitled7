use tokio::fs;
use tracing::{info, instrument};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
use tokio::sync::mpsc::channel;

#[instrument]
fn foo(ans: i32) {
    info!("in foo");
}



#[derive(Debug,Clone)]
struct stu {
    url: Vec<String>,
}

fn main()  {
    tracing_subscriber::registry().with(fmt::layer()).init();
    println!("{:?}","haha");
    let stu1 = stu { url: vec!["1".to_string()] };
   let mut  s = "/sponge/server/v2/transaction/snapshot/traceInstanceLevel";
    let option = s.strip_prefix("/spxnge").unwrap_or_default();
   println!("{}",option);

    let (sender,recevcer) = channel(10);
}


