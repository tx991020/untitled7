use actix::prelude::*;



#[derive(Message)]
#[rtype(result = "()")]
struct Ping(String);

#[derive(Default)]
struct MyActor1;

impl Actor for MyActor1 {
    type Context = Context<Self>;
}


impl Handler<Ping> for MyActor1 {
    type Result = ();

    fn handle(&mut self, msg: Ping, ctx: &mut Context<Self>) {
        println!("{}","Actor1开始运行" );
        println!("{}  ping",msg.0);

        ctx.stop();
        System::current().stop();
    }
}


impl Supervised for MyActor1{
    fn restarting(&mut self, ctx: &mut Context<MyActor1>) {
        println!("restarting");
    }
}
impl ArbiterService for MyActor1{}




// Actor definition
#[derive(Default)]
struct Calculator;

impl Actor for Calculator {
    type Context = Context<Self>;
}


// now we need to implement `Handler` on `Calculator` for the `Sum` message.
impl Handler<Sum> for Calculator {
    type Result = usize; // <- Message response type

    fn handle(&mut self, msg: Sum, ctx: &mut Context<Self>) -> Self::Result {
        println!("{}","开始计算" );
        msg.0 + msg.1
    }
}


// Actor definition
#[derive(Default)]
struct MyActor3;

impl Actor for MyActor3 {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = MyActor1::from_registry();
        addr.do_send(Ping("haha".to_string()))

    }
}


// now we need to implement `Handler` on `Calculator` for the `Sum` message.
impl Handler<Sum> for MyActor3 {
    type Result = usize; // <- Message response type

    fn handle(&mut self, msg: Sum, ctx: &mut Context<Self>) -> Self::Result {
        println!("{}","开始计算" );
        msg.0 + msg.1
    }
}



#[derive(Message)]
#[rtype(result = "usize")]
struct Sum(usize, usize);



#[actix::main] // <- starts the system and block until future resolves
async fn main() {
    // let addr = Calculator.start();
    // // let addr1 = Calculator::from_registry();
    //
    // let res = addr.send(Sum(10, 5)).await; // <- send message and get future for result
    //
    // match res {
    //    Ok(result) => println!("SUM: {}", result),
    //    _ => println!("Communication to the actor has failed"),
    // }
    // let addr = Calculator.start();
    let addr3 = MyActor3.start();
    addr3.send(Sum(1,3)).await;
    // let addr = MyActor1::from_registry();
    // addr.send(Ping("haha".to_string())).await;


}