use std::sync::Arc;

use axum::extract::{Json, State};
use axum::{http::StatusCode, response::IntoResponse};
use domain::users::CreateUserRequest;
use serde::Deserialize;
use serde_json::json;

use crate::users::controller::UsersController;

// TODO: validate password
pub async fn create_user(
    State(ctl): State<Arc<UsersController>>,
    Json(req): Json<CreateUserRequest>,
) -> impl IntoResponse {
    match ctl.create(&req).await {
        Ok(new_user) => (StatusCode::OK, Json(json!(new_user))).into_response(),
        Err(message) => (StatusCode::CONFLICT, message).into_response(),
    }
}

#[derive(Debug, Deserialize)]
pub struct AuthUserRequest {
    _login: String,
    _password: String,
}

pub async fn login_user(
    State(_user_service): State<Arc<UsersController>>,
    Json(_req): Json<AuthUserRequest>,
) -> impl IntoResponse {
    todo!();
}
