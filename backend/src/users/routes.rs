use actix_web::{http::Error, post, web, HttpResponse};
use domain::users::CreateUserRequest;
use serde::Deserialize;

use crate::users::{controller::UsersController, users_service::UsersService};

// TODO: validate password
#[post("")]
pub async fn create_user(
    req: web::Json<CreateUserRequest>,
    ctl: web::Data<UsersController>,
) -> Result<HttpResponse, Error> {
    match ctl.create(&req) {
        Ok(new_user) => Ok(HttpResponse::Ok().json(new_user)),
        Err(message) => Ok(HttpResponse::Conflict().json(message)),
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
    _user_service: web::Data<UsersService>,
) -> Result<HttpResponse, Error> {
    todo!();
}
