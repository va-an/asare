use super::repository::{PortfolioRepoInMemory, PortfolioRepository, UserPortfolio};

pub struct PortfolioService {
    port_repo: PortfolioRepoInMemory,
}

impl PortfolioService {
    pub fn new() -> PortfolioService {
        let port_repo = PortfolioRepoInMemory::new();

        PortfolioService { port_repo }
    }

    pub fn find_by_user(&self, user_id: &i32) -> Vec<UserPortfolio> {
        self.port_repo.find_by_user(user_id)
    }
}
