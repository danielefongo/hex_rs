use std::fmt::Display;

use actix_web::{
    web::{self, Data, Json},
    App, HttpRequest, HttpResponse, HttpServer, ResponseError,
};
use auth::with_user;
use context::Context;
use domain::Name;
use serde::Deserialize;
use serde_json::json;

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
    GetUser(String),
}

pub async fn process_command(
    request: HttpRequest,
    state: Data<Context>,
    data: Json<Command>,
) -> HttpResponse {
    let response = match data.into_inner() {
        Command::CreateUser(user) => {
            state.create_user_usecase().run(Name(user)).unwrap();

            json!({ "status": "Ok" })
        }
        Command::GetUser(user) => {
            let authenticated_user = request
                .headers()
                .get("user")
                .map(|it| it.to_str().unwrap_or_default().to_string());

            let user = with_user(authenticated_user, async move {
                state.get_user_usecase().run(Name(user)).unwrap()
            })
            .await;

            json!({ "name": user.name.to_string() })
        }
    };

    HttpResponse::Ok()
        .content_type("application/json")
        .body(response.to_string())
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
