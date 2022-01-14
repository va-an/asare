use crate::{
    app::PortfolioInteractor,
    portfolios::portfolios_service::{Portfolios, UserPortfolio},
    users::api_key_matcher::UserApiKeyMatcher,
};
use actix_web::{delete, get, http::Error, post, web, HttpRequest, HttpResponse};
use domain::Portfolio;
use serde::Serialize;

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

#[post("")]
pub async fn create(
    req: HttpRequest,
    create_request: web::Json<Portfolio>,
    portfolio_interactor: web::Data<PortfolioInteractor>,
) -> Result<HttpResponse, Error> {
    match extract_user_id(&req, &portfolio_interactor.api_key_matcher) {
        Ok(user_id) => {
            let new_portfolio = UserPortfolio::new(&user_id, &create_request);
            let created_portfolio = portfolio_interactor.portfolios.create(new_portfolio);
            let portfolio_response = UserPortfolioResponse::from(&created_portfolio);

            Ok(HttpResponse::Ok().json(portfolio_response))
        }
        Err(message) => Ok(HttpResponse::BadRequest().json(message)),
    }
}

#[get("")]
pub async fn find(
    req: HttpRequest,
    portfolio_interactor: web::Data<PortfolioInteractor>,
) -> Result<HttpResponse, Error> {
    match extract_user_id(&req, &portfolio_interactor.api_key_matcher) {
        Ok(user_id) => {
            let portfolios: Vec<UserPortfolioResponse> = portfolio_interactor
                .portfolios
                .find_by_user(&user_id)
                .iter()
                .map(|user_portfolio| UserPortfolioResponse::from(user_portfolio))
                .collect();

            Ok(HttpResponse::Ok().json(portfolios))
        }
        Err(message) => Ok(HttpResponse::BadRequest().json(message)),
    }
}

#[delete("")]
pub async fn delete() -> Result<HttpResponse, Error> {
    todo!();
}

fn extract_user_id(req: &HttpRequest, api_key_matcher: &UserApiKeyMatcher) -> Result<i32, String> {
    match req.headers().get(API_KEY_HEADER) {
        Some(header_value) => {
            let api_key = header_value
                .to_str()
                .ok()
                .expect("Error with extracting header string");

            match api_key_matcher.find_user_id(api_key) {
                Some(user_id) => Ok(user_id),
                None => Err("Not found user by API key".to_string()),
            }
        }
        None => Err("Not found API key header".to_string()),
    }
}
