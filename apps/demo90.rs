
use lazy_static::lazy_static;
use tokio::signal::ctrl_c;
use tokio::sync::mpsc::channel;
use reqwest;
use serde_json::json;


#[derive(Default, Debug)]
struct Stu<'a> {
    name: &'a str,
    age: i32,
}


impl<'a> Stu<'a> {
    fn new(name: &'a str, age: i32) -> Self {
        Self {
            name,
            age,
        }
    }
    pub fn set_name(&mut self, name: &'a str) -> &mut Self {
        self.name = name;
        self
    }
    pub fn get_name(&self) -> &str {
        self.name
    }
    pub fn get_age(&self) -> i32 {
        self.age
    }
}

lazy_static! {
    static ref STUDENT: Stu<'static> = Stu::new("张三", 20);
}

#[tokio::main]
async  fn main() {
    let client1 = reqwest::Client::new();
    let x = client1.post("http://www.baidu.com").json(&json!(
        {
            "name": "张三",
            "age": 20,
        }
    )).send().await.json().await.unwrap();




}