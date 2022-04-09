
extern crate derive_more;
// use the derives that you want in the file
use derive_more::{Add, Display, From,Deref,DerefMut,Into};
use std::collections::HashMap;

#[derive(Deref, DerefMut,Display,Debug)]
struct Num {
    num: i32,
}

#[derive(Deref, DerefMut,Display,Debug)]
struct Pool(i32);

#[derive(Deref, DerefMut)]
#[deref(forward)]
#[deref_mut(forward)]
struct MyBoxedInt(Box<i32>);

// You can specify the field you want to derive DerefMut for
#[derive(Deref, DerefMut)]
struct CoolVec {
    cool: bool,
    #[deref]
    #[deref_mut]
    vec: Vec<i32>,
}


#[derive(Deref, DerefMut,Debug)]
#[deref(forward)]
#[deref_mut(forward)]
struct MyBox<T>(Box<T>);




impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(Box::new(x))
    }
    async fn todo (){

    }
}

#[derive(PartialEq, From, Add, Display)]
enum MyEnum {
    #[display(fmt = "int: {}", _0)]
    Int(i32),
    Uint(u32),
    #[display(fmt = "nothing")]
    Nothing,
}

#[derive(PartialEq, From, Into,Debug)]
struct Point2D {
    x: i32,
    y: i32,
}




fn main() {
    let mut num = Num{num: 123};
    // let mut boxed = MyBoxedInt(Box::new(123));
    // let mut cool_vec = CoolVec{cool: true, vec: vec![123]};
    *num += 123;
    println!("{:?}",num );

    let mut m = MyBox::new(String::from("Rust"));

    m.push_str("wowow");
    println!("{:?}",m );
    let mut pool = Pool(1);
    *pool +=1;

    let x :(i32,i32) = Point2D { x: 5, y: 6 }.into();
    println!("{:?}",x );

    let d = Point2D::from((1, 2));
    println!("111{:?}",d );

    // let step =0;
    // match step {
    //     0 => {
    //         println!("{}","0000")
    //     }
    //     _=>{
    //         println!("{}","1111")
    //     }
    // }

   // main1();
   //  let mut map = HashMap::new();
   //    map.insert("a",1);
   //    map.insert("b",2);
   //  let mut map1 = HashMap::new();
   //  for (k,v) in map.iter() {
   //      k
   //  }

    main2();



    // assert_eq!(246, *num);
    // *boxed += 1000;
    // assert_eq!(1123, *boxed);
    // cool_vec.push(456);
    // assert_eq!(vec![123, 456], *cool_vec);
}




fn convert(v: &Vec<i32>) -> String {
    format!("{:?}", v)
}

fn main2() {
    let mut old: HashMap<String, Vec<i32>> = HashMap::new();
    old.insert("a".to_string(),vec![1,2,3]);
    let new: HashMap<_, _> = old
        .iter()
        .map(|(key, value)| (key.clone(), convert(value)))
        .collect();
    println!("{:?}", new);
}


fn get_aliases(aliases: HashMap<String, String>) -> Vec<(String, Vec<String>)> {
    let mut x = HashMap::new();

    for (k, v) in aliases {
        x.entry(v).or_insert_with(Vec::new).push(k)
    }

    x.into_iter().collect()
}





fn main1() {
    let mut aliases = HashMap::new();
    aliases.insert("a".to_owned(), "test".to_owned());
    aliases.insert("b".to_owned(), "test".to_owned());
    aliases.insert("c".to_owned(), "test2".to_owned());

    println!("{:?}", get_aliases(aliases));
}