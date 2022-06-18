use super::service::UserPortfolio;

pub trait PortfolioRepository {
    fn create(&self, portfolio: &UserPortfolio) -> UserPortfolio;

    fn find_by_id(&self, id: &i32) -> UserPortfolio;
    fn find_by_user(&self, user_id: &i32) -> Vec<UserPortfolio>;

    fn delete_by_id(&self, id: &i32);
}
