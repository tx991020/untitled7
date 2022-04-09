use actix::prelude::*;
use actix_broker::{Broker, BrokerSubscribe, SystemBroker};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use std::io;

use std::thread::sleep;
use std::time::Duration;
use chrono::prelude::*;

#[derive(Clone, Debug, Message)]
#[rtype(result = "()")]
struct World1 {
    name : String
}


struct TestActor1;



impl Actor for TestActor1{
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {

        self.subscribe_async::<SystemBroker, World1>(ctx);
        self.subscribe_async::<SystemBroker, World2>(ctx);




    }
}

impl Handler<World1> for TestActor1 {
    type Result = ();
    fn handle(&mut self, msg: World1, _ctx: &mut Self::Context) {
       // web::block(
       //     move || {
       //         sleep(Duration::from_secs(10));
       //         println!("TestActor: Received {:?},{}",msg.name, Local::now().timestamp());
       //     }
       // );
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(10)).await;

            println!("{},{}", msg.name ,Local::now().timestamp() );
        });
    }
}



#[derive(Clone, Debug, Message)]
#[rtype(result = "()")]
struct World2 {
    name : String
}



impl Handler<World2> for TestActor1 {
    type Result = ();
    fn handle(&mut self, msg: World2, _ctx: &mut Self::Context) {
     web::block(move || {
         sleep(Duration::from_secs(3));
         println!("TestActor: Received {:?},{}",msg.name, Local::now().timestamp());
     });
    }
}





async fn index(_req: HttpRequest) -> Result<HttpResponse, Error> {
    println!("TestActor: Received {:?},{}","start", Local::now().timestamp());
        Broker::<SystemBroker>::issue_async(World1{ name: "zhangsan".to_string() });
        Broker::<SystemBroker>::issue_async(World2{ name: "lisi".to_string() });



    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body("Welcome!"))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let _addr = TestActor1.start();


    HttpServer::new(|| {
        App::new()
            .service(web::scope("/").service(web::resource("").route(web::get().to(index))))
    })
        .bind("127.0.0.1:8082")
        .unwrap()
        .run()
        .await
}


// extern crate riker;
// use riker::actors::*;
//
// use std::time::Duration;
//
// #[derive(Clone, Debug)]
// pub struct Panic;
//
// #[derive(Default)]
// struct DumbActor;
//
// impl Actor for DumbActor {
//     type Msg = ();
//
//     fn recv(&mut self, _: &Context<Self::Msg>, _: Self::Msg, _: Sender) {}
// }
//
// #[actor(Panic)]
// #[derive(Default)]
// struct PanicActor;
//
// impl Actor for PanicActor {
//     type Msg = PanicActorMsg;
//
//     fn pre_start(&mut self, ctx: &Context<Self::Msg>) {
//         ctx.actor_of::<DumbActor>("child_a").unwrap();
//
//         ctx.actor_of::<DumbActor>("child_b").unwrap();
//
//         ctx.actor_of::<DumbActor>("child_c").unwrap();
//
//         ctx.actor_of::<DumbActor>("child_d").unwrap();
//     }
//
//     fn recv(&mut self, ctx: &Context<Self::Msg>, msg: Self::Msg, sender: Sender) {
//         self.receive(ctx, msg, sender);
//     }
// }
//
// impl Receive<Panic> for PanicActor {
//     type Msg = PanicActorMsg;
//
//     fn receive(&mut self, _ctx: &Context<Self::Msg>, _msg: Panic, _sender: Sender) {
//         panic!("// TEST PANIC // TEST PANIC // TEST PANIC //");
//     }
// }
//
// fn main() {
//     let sys = SystemBuilder::new().name("my-app").create().unwrap();
//
//     let sup = sys.actor_of::<PanicActor>("panic_actor").unwrap();
//     // println!("Child not added yet");
//     // sys.print_tree();
//
//     println!("Before panic we see supervisor and actor that will panic!");
//     std::thread::sleep(Duration::from_millis(500));
//     sys.print_tree();
//
//     sup.tell(Panic, None);
//     std::thread::sleep(Duration::from_millis(500));
//     println!("We should see panic printed, but we still alive and panic actor gone!");
//     sys.print_tree();
// }
