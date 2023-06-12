use std::fmt::Display;

use actix_web::{
    web::{self, Data, Json},
    App, HttpResponse, HttpServer, ResponseError,
};
use context::Context;
use domain::Name;
use serde::Deserialize;

#[derive(Debug)]
pub struct Error;
impl ResponseError for Error {}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error")
    }
}

#[derive(Debug, Deserialize)]
pub enum Command {
    CreateUser(String),
}

pub async fn process_command(state: Data<Context>, request: Json<Command>) -> HttpResponse {
    match request.into_inner() {
        Command::CreateUser(user) => state.create_user_usecase().run(Name(user)).unwrap(),
    };

    HttpResponse::Ok()
        .content_type("application/json")
        .body("Ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(Context {}))
            .route("/", web::post().to(process_command))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
