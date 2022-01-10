use std::sync::Arc;

use actix_web::{http::Error, post, web, HttpResponse};
use serde::Deserialize;

use crate::entities::users::{CreateUserRequest, Users};

// TODO: validate password
#[post("")]
pub async fn create_user(
    req: web::Json<CreateUserRequest>,
    user_service: web::Data<Arc<Users>>,
) -> Result<HttpResponse, Error> {
    let new_user = user_service.create_user(&req);

    Ok(HttpResponse::Ok().json(new_user))
}

#[derive(Debug, Deserialize)]
struct AuthUserRequest {
    login: String,
    password: String,
}

#[post("/refresh_api_key")]
pub async fn login_user(
    req: web::Json<AuthUserRequest>,
    user_service: web::Data<Users>,
) -> Result<HttpResponse, Error> {
    todo!();
}
