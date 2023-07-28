use std::fmt::Display;

use actix_web::{
    web::{self, Data, Json},
    App, HttpRequest, HttpResponse, HttpServer, ResponseError,
};
use auth::with_user;
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

pub async fn process_command(
    request: HttpRequest,
    state: Data<Context>,
    data: Json<Command>,
) -> HttpResponse {
    let authenticated_user = request
        .headers()
        .get("user")
        .map(|it| it.to_str().unwrap_or_default().to_string());

    match data.into_inner() {
        Command::CreateUser(user) => {
            with_user(authenticated_user, async move {
                state.create_user_usecase().run(Name(user)).unwrap()
            })
            .await
        }
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
