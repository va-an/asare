use async_trait::async_trait;

use super::service::UserPortfolio;

#[async_trait]
pub trait PortfolioRepository {
    async fn create(&self, portfolio: &UserPortfolio) -> UserPortfolio;

    async fn find_by_id(&self, id: i32) -> UserPortfolio;
    async fn find_by_user(&self, user_id: i32) -> Vec<UserPortfolio>;

    async fn delete_by_id(&self, id: i32, user_id: i32);
}
