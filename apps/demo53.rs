use actix_web::{get, web, App, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct MyObj {
    name: String,
}

#[get("/{id}/{name}")]
async fn index(info: web::Path<(u32, String)>) -> impl Responder {
    let info = info.into_inner();
    let obj1 = MyObj { name: info.1 };
    web::Json(obj1)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}