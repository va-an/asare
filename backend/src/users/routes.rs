use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use domain::users::CreateUserRequest;
use serde::Deserialize;
use serde_json::json;

use crate::users::{controller::UsersController, service::UserService};

// TODO: validate password
pub async fn create_user(
    Json(req): Json<CreateUserRequest>,
    Extension(ctl): Extension<Arc<UsersController>>,
) -> impl IntoResponse {
    match ctl.create(&req) {
        Ok(new_user) => (StatusCode::OK, Json(json!(new_user))),
        Err(message) => (StatusCode::CONFLICT, Json(json!(message))),
    }
}

#[derive(Debug, Deserialize)]
pub struct AuthUserRequest {
    _login: String,
    _password: String,
}

pub async fn login_user(
    Json(_req): Json<AuthUserRequest>,
    Extension(_user_service): Extension<Arc<UserService>>,
) -> impl IntoResponse {
    todo!();
}
