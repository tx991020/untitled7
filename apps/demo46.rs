use std::fmt::{Display, Formatter};
use futures::future::{BoxFuture, FutureExt};
use protobuf::reflect::ProtobufValue;
use std::ptr;

#[macro_use]
extern crate derive_more;

macro_rules! dyn_async {( $($contents:tt)* ) => (
    FutureExt::boxed(async move {
        $($contents)*
    })
)}

#[derive(Debug)]
struct S {}

struct P {}

impl P {
    pub async fn f1<F>(&self, callback: F) -> ()
    where
        F: for<'a> Fn(&'a S) -> BoxFuture<'a, ()>,
    {
        let s = S {};
        callback(&s).await;
    }
}

fn foo< T>(f: T)
where
    T: for<'a> Fn(&'a i32),
{
    let x = 1;
    f(&x);
    {
        let y = 2;
        f(&y);
    }
}

impl Display for class{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}



#[derive(AsRef)]
#[derive(Display)]
#[display(fmt = "{}",first)]
struct SingleFieldStruct {
    first: String,
}


#[derive(Display)]
#[display(fmt = "({}, {})", x, y)]
struct Point2D {
    x: String,
    y: String,
}


#[derive(AsRef)]
#[derive(Display)]
#[display(fmt = "{:?}", first)]
struct MultiFieldGenericStruct<T> {
    #[as_ref]
    first: Vec<T>,

}

// impl AsRef<class> for class {
//     fn as_ref(&self) -> &class {
//         self
//     }
// }

#[derive(Debug)]
pub struct class {
    name: String,
}

fn test<T>(x :&[T]) where T:Display + std::fmt::Debug{
    println!("{:?}",x );
}

fn f<T>(x: &[T]) where T: AsRef<class> + std::fmt::Debug{
    println!("{:?}",x );
}

struct A {
    x :String,
    y :i32,
}


#[derive(Debug)]
struct B {
    x: String,
    y: i32,
    z: String,
}

impl From<A> for B {
    fn from(x: A) -> Self {
        Self{
            x: x.x,
            y: x.y,
            z:"".to_string()
        }
    }
}

fn sInto<T>(x:T) where T: Into<String> + std::fmt::Debug {
    println!("{:?}",x );
}


use lazy_static::lazy_static;


fn main() {
    let obj = P {};
    obj.f1(|s: &S| {
        dyn_async! {
           println!("{:?}",s);
        }
    });
    let t = |x :&i32| {x;};

    foo(|x:&i32|{});
    // let x = vec![class{ name: "1".to_string() },class{ name: "2".to_string() }];
    //
    // let s = String::from("12300");
    //
    //
    //
    // f(&x);


    let item = SingleFieldStruct {
        first: String::from("test"),
    };
    println!("{}",item.as_ref() );
    let d = Point2D { x: "1".to_string(), y: "2".to_string() };
    println!("{}",d );
    //assert!(ptr::eq(&item.first, item.as_ref()));


    let item  = MultiFieldGenericStruct {
        first: b"test".to_vec(),

    };
    println!("{}",item );

    // let b = B::from(A { x: "1".to_string(), y: 2 });
    // dbg!(b);
    // let a = A{ x: "3".to_string(), y: 4 };
    // let x1 :B= a.into();
    // dbg!(x1);

    sInto("haha");
    sInto("wowo".to_string())

}
