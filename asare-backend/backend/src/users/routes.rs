use actix_web::{http::Error, post, web, HttpResponse};
use serde::Deserialize;

use crate::{app::UsersType, entities::users::CreateUserRequest};

// TODO: validate password
#[post("")]
pub async fn create_user(
    req: web::Json<CreateUserRequest>,
    users: web::Data<UsersType>,
) -> Result<HttpResponse, Error> {
    match users.create(&req) {
        Ok(new_user) => Ok(HttpResponse::Ok().json(new_user)),
        Err(message) => Ok(HttpResponse::InternalServerError().json(message)),
    }
}

#[derive(Debug, Deserialize)]
pub struct AuthUserRequest {
    _login: String,
    _password: String,
}

#[post("/refresh_api_key")]
pub async fn login_user(
    _req: web::Json<AuthUserRequest>,
    _user_service: web::Data<UsersType>,
) -> Result<HttpResponse, Error> {
    todo!();
}
