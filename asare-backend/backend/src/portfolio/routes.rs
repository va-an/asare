use crate::app::PortfolioInteractor;
use actix_web::{delete, get, http::Error, post, web, HttpRequest, HttpResponse};
use serde::Deserialize;

static API_KEY_HEADER: &str = "X-Api-Key";

#[derive(Debug, Deserialize)]
pub struct CreatePortfolioRequest;

#[post("")]
pub async fn create(
    req: web::Json<CreatePortfolioRequest>,
    portfolio_interactor: web::Data<PortfolioInteractor>,
) -> Result<HttpResponse, Error> {
    todo!();
}

#[delete("")]
pub async fn delete() -> Result<HttpResponse, Error> {
    todo!();
}

#[get("")]
pub async fn find(
    req: HttpRequest,
    portfolio_interactor: web::Data<PortfolioInteractor>,
) -> Result<HttpResponse, Error> {
    match extract_api_key(&req) {
        Some(api_key) => {
            let user_id = portfolio_interactor
                .api_key_matcher
                .find_user_id(api_key)
                .expect("Not found user by API key");

            let portfolios = portfolio_interactor.service.find_by_user(&user_id);

            Ok(HttpResponse::Ok().json(portfolios))
        }
        None => Ok(HttpResponse::BadRequest().json(format!("Not found header {}", API_KEY_HEADER))),
    }
}

fn extract_api_key(req: &HttpRequest) -> Option<&str> {
    match req.headers().get(API_KEY_HEADER) {
        Some(header_value) => header_value.to_str().ok(),
        None => None,
    }
}
