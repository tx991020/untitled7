

use std::time::Duration;
use smol::stream::StreamExt;
use smol::channel::RecvError;

fn main() {
    println!("Hello, world!");

    smol::block_on(
        async move {
            // let task = smol::spawn(async move {
            //     smol::Timer::after(Duration::from_secs(10)).await;
            //     println!("{}","haha" );
            // });
            // let task1 = smol::spawn(async move {
            //     smol::Timer::after(Duration::from_secs(5)).await;
            //     println!("{}","wowo" );
            // });
            //
            // smol::future::zip(task,task1).await;


            let (s,r) = smol::channel::bounded(10);
            for i in  1..5 {
                s.send(i).await;
            }
            s.close();


            while let x= r.clone().recv().await.map_or(0, |e|{e}) {
                println!("{}",x );
            };
            println!("{}",666 );
            // r.recv().await.



            // let x1 = smol::stream::iter(1..3).map(|x| { x + 1 }).collect().await;
        }
    )
}
