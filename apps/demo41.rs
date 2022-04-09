


use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;


use  tokio;
use lazy_static::lazy_static;
use dashmap::DashMap;
use tokio::signal::ctrl_c;
use tokio::{task, time};

type Map =  Arc<DashMap<String, i8>>;



lazy_static::lazy_static! {

    pub static ref IdMap: Map = Arc::new(DashMap::new());

}



fn main() {


    let threads: Vec<_> = (0..10).map(|i| {
        thread::spawn(move ||todo1(i, IdMap.clone()))
    }).collect();

    let thread1: Vec<_> = (10..20).map(|i| {
        thread::spawn(move ||todo3(i, IdMap.clone()))
    }).collect();
    println!("{}", 11);
    // for thread in threads {
    //     thread.join();
    // }
    // for thread in thread1 {
    //     thread.join();
    // }

    thread::spawn(move ||todo2(IdMap.clone()));
    println!("{}",111 );
    thread::sleep(Duration::from_secs(3));
    println!("{:?}", IdMap.clone());


}

#[tokio::main]
async fn todo1(i:i8,t :Map){
    let option = t.get("a");
    if option.is_none() {
        t.insert(format!("{}","a"),i);
    }else {
        let mut v = option.as_ref().unwrap().value();
        t.insert("a".to_string(),*v+i);
    }


}

#[tokio::main(flavor = "current_thread")]
async fn todo3(i:i8,t :Map){
    let option = t.get("a");
    if option.is_none() {
        t.insert(format!("{}","a"),i);
    }else {
        let mut v = option.as_ref().unwrap().value();
        t.insert("a".to_string(),*v+i);
    }

}
#[tokio::main(flavor = "current_thread")]
async fn todo2(t :Map){
    let mut interval = time::interval(Duration::from_secs(3));
    loop {
        println!("{:?},{}",t.clone(),t.len() );

        interval.tick().await;
    }
}