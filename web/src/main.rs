use std::fmt::Display;

use actix_web::{
    web::{self, Data},
    App, HttpResponse, HttpServer, ResponseError,
};

#[derive(Debug)]
pub struct Error;
impl ResponseError for Error {}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error")
    }
}

#[derive(Debug)]
pub struct Context {}

pub async fn foo(state: Data<Context>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(format!("{:?}", state))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(Context {}))
            .route("/", web::get().to(foo))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
