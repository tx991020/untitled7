use actix::prelude::*;
use reqwest;

// this is our Message
#[derive(Message)]
#[rtype(result = "reqwest::Result<String>")]
struct HttpPing(String);



#[derive(Message)]
#[rtype(result = "usize")]
struct Ping {
    pub id: usize,
}



struct HttpPinger1{

}

// Actor definition
struct HttpPinger{

}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Line {
    line: String,
}

impl Actor for HttpPinger {
    type Context = Context<Self>;
}


impl Actor for Ping {
    type Context = Context<Self>;
}


impl Handler<Ping> for HttpPinger{
    type Result = usize;

    fn handle(&mut self, msg: Ping, ctx: &mut Self::Context) -> Self::Result {
        println!("11111 {}",msg.id);
        return msg.id ;
    }
}



impl Handler<HttpPing> for HttpPinger {
    type Result = reqwest::Result<String>; // <- Message response type

    fn handle(&mut self, msg: HttpPing, _ctx: &mut Context<Self>) -> Self::Result {

        //type Result = AtomicResponse<Self, usize>;

        // AtomicResponse::new(Box::pin(
        //     sleep(Duration::from_millis(msg.0 as u64))
        //         .into_actor(self)
        //         .map(move |_res, this, _| {
        //             this.0 += msg.0;
        //             this.0
        //         }),
        // ))
        let recipient = _ctx.address().recipient();
        let future = async move {
            let line= reqwest::get(msg.0).await.unwrap().text().await.unwrap();
            println!("{:?}",&line);
            recipient.do_send(Ping { id: 1 });

        };
        future.into_actor(self).spawn(_ctx);




        //  use async reqwest? is that possible, maybe w/ tokio::spawn()?
        /*
        let body = tokio::spawn(async move {
            let client = reqwest::Client::builder().build().unwrap();
            let body = client.get(msg.0).send().await.unwrap().text().await.unwrap();
            body
        });
        */

        Ok("haha".to_string())
    }
}

#[actix::main]
async fn main() {


    let addr = HttpPinger{ }.start();
    let res = addr.send(HttpPing("https://www.rust-lang.org".to_string())).await;



    match res {
        Ok(Ok(result)) => println!("body: {}", result),
        _ => println!("Actor has failed"),
    }
}