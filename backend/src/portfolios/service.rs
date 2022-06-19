use domain::Portfolio;
use serde::{Deserialize, Serialize};

use super::{repository::PortfolioRepository, repository_in_memory::PortfolioRepoInMemory};

pub trait Portfolios {
    fn create(&self, portfolio: UserPortfolio) -> UserPortfolio;
    fn find_by_user(&self, user_id: &i32) -> Vec<UserPortfolio>;
}

pub struct PortfoliosImpl {
    port_repo: PortfolioRepoInMemory,
}

impl PortfoliosImpl {
    pub fn new() -> PortfoliosImpl {
        let port_repo = PortfolioRepoInMemory::new();

        PortfoliosImpl { port_repo }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct UserPortfolio {
    pub id: i32,
    pub user_id: i32,
    pub portfolio: Portfolio,
}

impl UserPortfolio {
    pub fn new(user_id: &i32, portfolio: &Portfolio) -> UserPortfolio {
        UserPortfolio {
            id: -1,
            user_id: user_id.to_owned(),
            portfolio: portfolio.to_owned(),
        }
    }
}

impl Portfolios for PortfoliosImpl {
    fn create(&self, portfolio: UserPortfolio) -> UserPortfolio {
        self.port_repo.create(&portfolio)
    }

    fn find_by_user(&self, user_id: &i32) -> Vec<UserPortfolio> {
        self.port_repo.find_by_user(user_id)
    }
}
