


use std::ops::{Deref, DerefMut};
struct MyBox<T>(T);

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
fn hello(name: &str) {
    println!("Hello, {}!", name);
}



fn main() {
    let mut m = MyBox::new(String::from("Rust"));

    m.push_str("wowow");
    hello(m.as_str())

}