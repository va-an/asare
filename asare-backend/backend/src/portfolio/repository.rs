use std::{collections::HashMap, sync::Mutex};

use super::portfolios_service::UserPortfolio;

pub trait PortfolioRepository {
    fn create(&self, portfolio: &UserPortfolio) -> UserPortfolio;

    fn find_by_id(&self, id: &i32) -> UserPortfolio;
    fn find_by_user(&self, user_id: &i32) -> Vec<UserPortfolio>;

    fn delete_by_id(&self, id: &i32);
}

pub struct PortfolioRepoInMemory {
    portfolios: Mutex<HashMap<i32, UserPortfolio>>,
    id_counter: Mutex<i32>,
}

impl PortfolioRepoInMemory {
    pub fn new() -> PortfolioRepoInMemory {
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

impl PortfolioRepository for PortfolioRepoInMemory {
    fn create(&self, user_portfolio: &UserPortfolio) -> UserPortfolio {
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

    fn find_by_id(&self, _id: &i32) -> UserPortfolio {
        todo!()
    }

    fn find_by_user(&self, user_id: &i32) -> Vec<UserPortfolio> {
        self.portfolios
            .lock()
            .unwrap()
            .values()
            .map(|p| p.to_owned())
            .filter(|p| &p.user_id == user_id)
            .collect()
    }

    fn delete_by_id(&self, _id: &i32) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        app::Portfolio,
        portfolio::{
            portfolios_service::UserPortfolio,
            repository::{PortfolioRepoInMemory, PortfolioRepository},
        },
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

    #[test]
    fn create_and_find_portfolio() {
        let port_repo = PortfolioRepoInMemory::new();
        let some_port = some_portfolio();

        let port_1 = port_repo.create(&some_port);
        let port_2 = port_repo.create(&some_port);

        assert_ne!(port_1, port_2);
    }
}
