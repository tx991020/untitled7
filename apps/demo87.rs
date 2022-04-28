use std::sync::Arc;
use std::thread;
use std::time::Duration;
use actix::prelude::*;

// this is our Message
// we have to define the response type (rtype)
#[derive(Message)]
#[rtype(result = "usize")]
struct Sum(usize, usize);



#[derive(Message)]
#[rtype(result = "()")]
struct Sum1(usize, usize);

// Actor definition
pub struct Calculator;

impl Actor for Calculator {
    type Context = Context<Self>;
}

// now we need to implement `Handler` on `Calculator` for the `Sum` message.
impl Handler<Sum> for Calculator {
    type Result = usize; // <- Message response type

    fn handle(&mut self, msg: Sum, _ctx: &mut Context<Self>) -> Self::Result {
        msg.0 + msg.1
    }
}


pub struct RecvActor {
    /// reply actor
    reply_addr: Addr<Calculator>,
}

impl RecvActor {
    pub fn new(reply_addr: Addr<Calculator>) -> Self {
        RecvActor {
            reply_addr,
        }
    }

    /// 解析mqtt的消息，并转换为执行指令
    fn parse_msg(&mut self, a:usize,b:usize ) {
        // 处理完业务逻辑，回复MQTT消息

        // 假设处理这个逻辑要等待5s，才能得到结果
        thread::sleep(Duration::from_millis(2000));
        println!("{}","haha");
        self.reply_addr.send(Sum(a,b));

    }
}


impl Actor for RecvActor {
    type Context = Context<Self>;
}

impl Handler<Sum1> for RecvActor {
    type Result = ();

    fn handle(&mut self, msg: Sum1, ctx: &mut Self::Context) -> Self::Result {
        // 在这里解析mqtt消息
        // self.parse_msg(msg.1);

        // 假如我在这里使用thread

            // info!("{}", msg.1);
            self.parse_msg(msg.0,msg.1);
    }
}


#[actix::main] // <- starts the system and block until future resolves
async fn main() {
    let addr = Calculator.start();



    let addx = RecvActor::new(addr).start();

    addx.send(Sum1(1, 2)).await;

    // match res {
    //     Ok(result) => println!("SUM: {}", result),
    //     _ => println!("Communication to the actor has failed"),
    // }


    // let res = addr.send(Sum(10, 5)).await; // <- send message and get future for result

    // match res {
    //     Ok(result) => println!("SUM: {}", result),
    //     _ => println!("Communication to the actor has failed"),
    // }
}