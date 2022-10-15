use std::{collections::HashMap, sync::Mutex};

use async_trait::async_trait;

use super::{repository::PortfolioRepository, service::UserPortfolio};

pub struct PortfolioRepoInMemory {
    portfolios: Mutex<HashMap<i32, UserPortfolio>>,
    id_counter: Mutex<i32>,
}

impl PortfolioRepoInMemory {
    pub fn new() -> Self {
        PortfolioRepoInMemory {
            portfolios: Mutex::new(HashMap::new()),
            id_counter: Mutex::new(0),
        }
    }

    fn next_id(&self) -> i32 {
        let next = *self.id_counter.lock().unwrap() + 1;
        *self.id_counter.lock().unwrap() = next;

        next
    }
}

#[async_trait]
impl PortfolioRepository for PortfolioRepoInMemory {
    async fn create(&self, user_portfolio: &UserPortfolio) -> UserPortfolio {
        let id = self.next_id();

        let new_portfolio = UserPortfolio {
            id,
            user_id: user_portfolio.user_id,
            portfolio: user_portfolio.portfolio.to_owned(),
        };

        self.portfolios
            .lock()
            .unwrap()
            .insert(id, new_portfolio.to_owned());

        new_portfolio
    }

    async fn find_by_id(&self, _id: i32) -> UserPortfolio {
        todo!()
    }

    async fn find_by_user(&self, user_id: i32) -> Vec<UserPortfolio> {
        self.portfolios
            .lock()
            .unwrap()
            .values()
            .map(|p| p.to_owned())
            .filter(|p| p.user_id == user_id)
            .collect()
    }

    async fn delete_by_id(&self, _id: i32, _user_id: i32) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use domain::Portfolio;

    use crate::portfolios::{
        repository::PortfolioRepository, repository_in_memory::PortfolioRepoInMemory,
        service::UserPortfolio,
    };

    fn some_portfolio() -> UserPortfolio {
        let portfolio = Portfolio::from([
            ("A".to_string(), 75_000.0),
            ("B".to_string(), 100_000.0),
            ("C".to_string(), 125_000.0),
        ]);

        UserPortfolio {
            id: -1,
            user_id: 1,
            portfolio,
        }
    }

    #[tokio::test]
    async fn create_and_find_portfolio() {
        let port_repo = PortfolioRepoInMemory::new();
        let some_port = some_portfolio();

        let port_1 = port_repo.create(&some_port).await;
        let port_2 = port_repo.create(&some_port).await;

        assert_ne!(port_1, port_2);
    }
}
