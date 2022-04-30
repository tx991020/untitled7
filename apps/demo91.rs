use std::convert::TryFrom;
use std::net::SocketAddr;
use axum::handler::Handler;
use axum::Router;
use axum::routing::get;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncReadExt;
pub struct Stuff {
    name: String,
    age: u32,
}

//构造 Stuff 实例 生成 get 和 set 方法
impl Stuff {
    fn new(name: String, age: u32) -> Stuff {
        Stuff {
            name,
            age,
        }
    }
    fn set_name(&mut self, name: String)->&mut Stuff {
        self.name = name;
        self

    }
    fn set_age(&mut self, age: u32) -> &mut Stuff {
        self.age = age;
        self
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn get_age(&self) -> u32 {
        self.age
    }
}

//tokio 执行shell命令 sterr 和 stdout
pub async fn shell(cmd: &str) -> Result<String, String> {
    let output = std::process::Command::new("sh").arg("-c").arg(cmd).output().await;
    if output.is_err() {
        return Err(format!("{:?}", output.err()));
    }
    let stdout = String::from_utf8(output.unwrap().stdout).unwrap();
    let stderr = String::from_utf8(output.unwrap().stderr).unwrap();
    if stderr != "" {
        return Err(stderr);
    }
    Ok(stdout)
}


//tokio 写文件
async fn write_file(path: &str, content: &str) {
    let mut file = File::create(path).await.unwrap();
    file.write_all(content.as_bytes()).await.unwrap();


}



//tokio 读文件 返回字符串
async fn read_file(path: &str) -> String {
    let mut file = File::open(path).await.unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).await.unwrap();
    buf

}

//TryFrom Stuff json 序列化
impl TryFrom<&str> for Stuff {
    type Error = String;
    fn try_from(json: &str) -> Result<Self, Self::Error> {
        let stuff: Stuff = serde_json::from_str(json)?;
        Ok(stuff)

    }
}
//TryFrom &Stuff 转 string
impl TryFrom<&Stuff> for String {
    type Error = String;
    fn try_from(stuff: &Stuff) -> Result<Self, Self::Error> {
        let json = serde_json::to_string(&stuff)?;
        Ok(json)
    }
}







// // json 序列化
// fn serialize_json(stuff: Stuff) -> String {
//     let stuff_json = serde_json::to_string(&stuff).unwrap();
//     stuff_json
//
// }
//
// // json 反序列化
// fn deserialize_json(stuff_json: &str) -> Stuff {
//     let stuff: Stuff = serde_json::from_str(stuff_json).unwrap();
//     stuff
//
// }


//
// async fn root() -> &'static str {
//     "Hello, world!"
// }
//
//
// async fn hello(name: String) -> String {
//     format!("Hello, {}!", name)
// }
//


fn main() {}