use crate::UserService;
use actix_web::{http::Error, post, web, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct AuthUserRequest {
    login: String,
    password: String,
}

// TODO: password may be optional - generate if None
// TODO: password check
#[post("/create")]
pub async fn create_user(
    req: web::Json<AuthUserRequest>,
    state: web::Data<UserService>,
) -> Result<HttpResponse, Error> {
    let login = &req.login;
    let password = &req.password;
    let new_user = state.create_user(login, password);

    Ok(HttpResponse::Ok().json(new_user))
}

#[post("/refresh_api_key")]
pub async fn login_user(
    req: web::Json<AuthUserRequest>,
    state: web::Data<UserService>,
) -> Result<HttpResponse, Error> {
    todo!();
}
