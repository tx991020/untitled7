use std::cell::Cell;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::mpsc::channel;
use tokio::sync::mpsc::unbounded_channel;
// use rayon::prelude::*;


fn main() {
    let (tx,rx) = channel();


    let (sender, mut receiver) = unbounded_channel();

    // 用rayon
    //自定义线程池
    // let _pool = rayon::ThreadPoolBuilder::new()
    //     .num_threads(10)
    //     .build()
    //     .unwrap();
    // let r = rand::random::<i32>() % 20;
    // println!("random:{:?}", r);
    // let mut data_p = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // data_p.par_iter_mut().for_each(|x| *x *= *x + r);
    // println!("data_p: {:?}", data_p);







    dbg!(y);

}