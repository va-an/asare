use actix_web::{delete, get, http::Error, post, web, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CreatePortfolioRequest;

#[post("/create")]
pub async fn create(req: web::Json<CreatePortfolioRequest>) -> Result<HttpResponse, Error> {
    todo!();
}

#[delete("/")]
pub async fn delete() -> Result<HttpResponse, Error> {
    todo!();
}

#[get("/")]
pub async fn find() -> Result<HttpResponse, Error> {
    todo!();
}
