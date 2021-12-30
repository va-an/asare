use actix_web::{http::Error, post, web, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CreateUserRequest {
    name: String,
}

#[derive(Debug, Deserialize)]
struct LoginUserRequest;

#[post("/create")]
pub async fn create_user(req: web::Json<CreateUserRequest>) -> Result<HttpResponse, Error> {
    todo!();
}

#[post("/login")]
pub async fn login_user(req: web::Json<LoginUserRequest>) -> Result<HttpResponse, Error> {
    todo!();
}
