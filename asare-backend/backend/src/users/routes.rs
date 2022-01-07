use crate::UserService;
use actix_web::{http::Error, post, web, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CreateUserRequest {
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserRequest;

#[post("/create")]
pub async fn create_user(
    req: web::Json<CreateUserRequest>,
    state: web::Data<UserService>,
) -> Result<HttpResponse, Error> {
    let new_user = state.create_user();

    Ok(HttpResponse::Ok().json(new_user))
}

#[post("/login")]
pub async fn login_user(
    req: web::Json<LoginUserRequest>,
    state: web::Data<UserService>,
) -> Result<HttpResponse, Error> {
    todo!();
}
