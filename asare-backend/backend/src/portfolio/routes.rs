use crate::app::PortfolioInteractor;
use actix_web::{delete, get, http::Error, post, web, HttpRequest, HttpResponse};
use serde::Deserialize;

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
    let api_key_header = "X-Api-Key";
    let api_key = req.headers().get(api_key_header);

    match api_key {
        Some(header) => {
            let header_value = header.to_str().expect("Error with extract header");

            let user_id = portfolio_interactor
                .api_key_matcher
                .find_user_id(header_value)
                .expect("Not found user by API key");

            let portfolios = portfolio_interactor.service.find_by_user(&user_id);

            Ok(HttpResponse::Ok().json(portfolios))
        }
        None => Ok(HttpResponse::BadRequest().json(format!("Not found header {}", api_key_header))),
    }
}
