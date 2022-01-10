use serde::Serialize;

use crate::{
    app::Portfolio,
    portfolio::repository::{PortfolioRepoInMemory, PortfolioRepository},
};

pub struct Portfolios {
    port_repo: PortfolioRepoInMemory,
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct UserPortfolio {
    pub id: i32,
    pub user_id: i32,
    pub portfolio: Portfolio,
}

impl Portfolios {
    pub fn new() -> Portfolios {
        let port_repo = PortfolioRepoInMemory::new();

        Portfolios { port_repo }
    }

    pub fn create(&self, portfolio: UserPortfolio) -> UserPortfolio {
        self.port_repo.create(&portfolio)
    }

    pub fn find_by_user(&self, user_id: &i32) -> Vec<UserPortfolio> {
        self.port_repo.find_by_user(user_id)
    }
}
