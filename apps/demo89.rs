use futures::{future::BoxFuture, Future};
use std::marker::Send;



type Callback = Box<dyn Fn() -> BoxFuture<'static, ()>>;

struct Store {
    fns: Vec<Callback>,
}

// impl Store {
//     fn add<T, F>(&mut self, cb: F)
//         where
//             F: Fn() -> T + 'static,
//             T: Future<Output = ()> + Send + 'static,
//     {
//         self.fns.push(Box::new(move || Box::pin(cb())));
//     }
// }

impl Store {
    fn add<T, F>(&mut self, cb: F)
        where
            F: Fn() -> T + 'static,
                T: Future<Output = ()> + Send + 'static,
    {
        self.fns.push(Box::new(move || Box::pin(cb())));
    }
}

async fn f1() -> () {
    println!("Hello");
}

async fn f2() -> () {
    println!("Goodbye");
}

fn main() {
    let mut s = Store { fns: Vec::new() };
    s.add(f1);
    s.add(f2);
}