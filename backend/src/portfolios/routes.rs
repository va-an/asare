use std::sync::Arc;

use crate::{
    app::PortfolioInteractor,
    portfolios::service::{Portfolios, UserPortfolio},
    users::api_key_matcher::UserApiKeyMatcher,
};
use axum::{
    extract::{Json, Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use domain::Portfolio;
use serde::Serialize;
use serde_json::json;

static API_KEY_HEADER: &str = "X-Api-Key";

#[derive(Debug, Serialize)]
struct UserPortfolioResponse {
    id: i32,
    portfolio: Portfolio,
}

impl UserPortfolioResponse {
    fn from(user_portfolio: &UserPortfolio) -> UserPortfolioResponse {
        UserPortfolioResponse {
            id: user_portfolio.id,
            portfolio: user_portfolio.portfolio.to_owned(),
        }
    }
}

pub async fn create(
    State(portfolio_interactor): State<Arc<PortfolioInteractor>>,
    headers: HeaderMap,
    Json(create_request): Json<Portfolio>,
) -> impl IntoResponse {
    match extract_user_id(&headers, &portfolio_interactor.api_key_matcher).await {
        Ok(user_id) => {
            let new_portfolio = UserPortfolio::new(&user_id, &create_request);
            let created_portfolio = portfolio_interactor.portfolios.create(new_portfolio).await;
            let portfolio_response = UserPortfolioResponse::from(&created_portfolio);

            (StatusCode::OK, Json(json!(portfolio_response))).into_response()
        }
        Err(message) => (StatusCode::BAD_REQUEST, message).into_response(),
    }
}

pub async fn find(
    headers: HeaderMap,
    State(portfolio_interactor): State<Arc<PortfolioInteractor>>,
) -> impl IntoResponse {
    match extract_user_id(&headers, &portfolio_interactor.api_key_matcher).await {
        Ok(user_id) => {
            let portfolios: Vec<UserPortfolioResponse> = portfolio_interactor
                .portfolios
                .find_by_user(user_id)
                .await
                .iter()
                .map(UserPortfolioResponse::from)
                .collect();

            (StatusCode::OK, Json(json!(portfolios))).into_response()
        }
        Err(message) => (StatusCode::BAD_REQUEST, message).into_response(),
    }
}

pub async fn delete(
    State(portfolio_interactor): State<Arc<PortfolioInteractor>>,
    headers: HeaderMap,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match extract_user_id(&headers, &portfolio_interactor.api_key_matcher).await {
        Ok(user_id) => {
            portfolio_interactor.portfolios.delete(id, user_id).await;
            StatusCode::OK.into_response()
        }
        Err(message) => (StatusCode::BAD_REQUEST, message).into_response(),
    }
}

async fn extract_user_id(
    headers: &HeaderMap,
    api_key_matcher: &UserApiKeyMatcher,
) -> Result<i32, String> {
    match headers.get(API_KEY_HEADER) {
        Some(header_value) => {
            let api_key = header_value
                .to_str()
                .expect("Error with extracting header string");

            match api_key_matcher.find_user_id(api_key).await {
                Some(user_id) => Ok(user_id),
                None => Err("Not found user by API key".to_string()),
            }
        }
        None => Err("Not found API key header".to_string()),
    }
}
