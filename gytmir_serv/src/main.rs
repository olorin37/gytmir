use actix_web::{get, web, App, HttpServer, HttpResponse, Responder};
use gytmir_lib::sync;
use std::thread;

#[get("/")]
async fn index() -> impl Responder {
    thread::spawn(|| {
        sync("../../doxtractor", "origin", ".ssh/id_rsa", "master");
    });
    HttpResponse::Accepted()

}        

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!\n", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
