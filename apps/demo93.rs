#![feature(trait_alias)]

use futures_util::future::BoxFuture;
use std::future::Future;
use std::task::Poll;

trait callback2<'a> {
    fn call(&self, ctx: &'a str) -> BoxFuture<'a, ()>;
}

// impl<'a, F, T> callback2<'a> for F
// where
//     F: Fn(&'a str) -> T,
//     T: Future<Output = ()> + Send + 'a,
// {
//     fn call(&self, ctx: &'a str) -> BoxFuture<'a, ()> {
//         Box::pin(self(ctx))
//     }
// }

async fn call<'a, T, F>(cb: T, y: &'a str)
where
    T: Fn(&'a str) -> F,
    F: Future<Output = ()> + Send + 'a,
{
    cb(y).await;
}

#[tokio::main]
async fn main() {




    // call(
    //     |x| {
    //         let string = x.to_string();
    //
    //         Box::pin(async move {
    //             println!("{}", string);
    //         })
    //     },
    //     "hah",
    // )
    //     .await;
}
