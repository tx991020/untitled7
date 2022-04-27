

use chrono::{Local, Utc};
use nanoid::{alphabet, nanoid};


pub fn genid(len: usize) -> String {
    nanoid!(len, &alphabet::SAFE[2..])
}

fn main() {
    let x = Local::now().timestamp();
    let i = Utc::now().timestamp();
    println!("{},{}",x,i);

    let string = genid(8);
    println!("{}",string)



}