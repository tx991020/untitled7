use std::rc::Rc;
use std::sync::Mutex;

#[derive(Default)]
struct Timer<'a> {
    callbacks: Mutex<Vec<Box<dyn Fn()+'a>>>,
}

impl<'a> Timer<'a> {
    fn add(&self, callback: Box<dyn Fn() + 'a>) {
        let mut callbacks = self.callbacks.lock().unwrap();

        callbacks.push(callback);
    }
}



#[derive(Default)]
struct Foo<'a> {
    value: Rc<usize>,
    timer: Timer<'a>,
}

impl<'a> Foo<'a> {
    fn process(&self) {
        let value_rc = self.value.clone();
        let callback = move || {
            println!("value is {}", *value_rc);
        };

        self.timer.add(Box::new(callback));
    }
}



// #[derive(Default)]
// struct Foo<'a> {
//     value: usize,
//     timer: Timer<'a>,
// }
//
// impl<'a> Foo<'a> {
//     fn callback(&self) {
//         println!("value is {}", self.value);
//     }
//
//     fn process(&self) {
//         let callback = || {
//             self.callback();
//         };
//
//         self.timer.add(Box::new(callback));
//     }
// }


struct A {}

impl A {
    fn finish(&self, msg: String) {
        println!("{}", msg);
    }

    fn test(&self) {
        let fun = |msg: String| self.finish(msg);
        let b = B::new(&fun);
        b.start("hi".to_string().clone());
    }
}

struct B<'b> {
    cb: &'b Fn(String),
}

impl<'b> B<'b> {
    fn new(cb: &'b Fn(String)) -> B<'b> {
        B { cb }
    }

    fn start(&self, msg: String) {
        (self.cb)(msg);
    }
}

// fn main() {
//     let a = A {};
//     a.test();
// }

fn main() {
    let foo = Foo::default();

    foo.process();
}