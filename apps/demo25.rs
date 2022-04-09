use tokio::sync::{broadcast, watch, Notify};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let (tx, mut rx1) = broadcast::channel(16);
    let mut rx2 = tx.subscribe();

    tokio::spawn(async move {
        assert_eq!(rx1.recv().await.unwrap(), 10);
        assert_eq!(rx1.recv().await.unwrap(), 20);
    });

    tokio::spawn(async move {
        println!(
            "broadcast {},{}",
            rx2.recv().await.unwrap(),
            rx2.recv().await.unwrap()
        );
    });

    tx.send(10).unwrap();
    tx.send(20).unwrap();

    let (tx3, mut rx3) = watch::channel("hello");

    tokio::spawn(async move {

        tokio::select! {
         _ = rx3.changed() => {
                        println!("{}","changed");
                        let conf = rx3.borrow().clone();
                        dbg!(conf);
                    }
    }
    });

    // tokio::spawn(async move {
    //     while let Some(value) = rx3.recv().await {
    //         println!("received = {:?}", value);
    //     }
    // });
    println!("{}",111 );
    tx3.send("hello").unwrap();
    println!("{}",444 );

    main1().await;

    tokio::signal::ctrl_c().await;
}




async fn main1() {
    let notify = Arc::new(Notify::new());
    let notify2 = notify.clone();

    let notified1 = notify.notified();
    let notified2 = notify.notified();

   tokio::spawn(async move {
        println!("sending notifications //通知所有人");
        notify2.notify_waiters();
    });

    notified1.await;
    println!("{}","aaa" );
    notified2.await;
    println!("{}","bbb" );
    println!("received notifications");
}