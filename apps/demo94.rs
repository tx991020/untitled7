use futures::{future::BoxFuture, Future};
use std::{marker::Send, sync::Arc};

struct Context {
    body: String,
}

trait Callback<'a> {
    fn call(&self, ctx: &'a mut Context) -> BoxFuture<'a, ()>;
}




impl<'a, T, F> Callback<'a> for F
    where
        F: Fn(&'a mut Context) -> T,
        T: Future<Output = ()> + Send + 'a,
{
    fn call(&self, ctx: &'a mut Context) -> BoxFuture<'a, ()> {
        Box::pin(self(ctx))
    }
}

struct Store {
    gets: Vec<(String, Arc<Box<dyn for<'a> Callback<'a>>>)>,
}

impl Store {
    fn get<F>(&mut self, path: &str, callback: F)
        where
            F: for<'a> Callback<'a> + 'static,
    {
        self.gets
            .push((path.to_string(), Arc::new(Box::new(callback))));
    }
}

async fn index(ctx: &mut Context) {
    ctx.body = "Hello World".to_string();
    println!("Hello");
}

fn main() {
    let mut s = Store { gets: Vec::new() };
    s.get("/", index);
}
