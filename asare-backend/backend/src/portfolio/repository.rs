use crate::app::Portfolio;
use serde::Serialize;
use std::{collections::HashMap, sync::Mutex};

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct UserPortfolio {
    id: i32,
    user_id: i32,
    portfolio: Portfolio,
}

pub trait PortfolioRepository {
    fn create(&self, user_id: &i32, portfolio: &Portfolio) -> UserPortfolio;

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
    fn create(&self, user_id: &i32, portfolio: &Portfolio) -> UserPortfolio {
        let id = self.next_id();
        self.portfolios.lock().unwrap().insert(
            id,
            UserPortfolio {
                id,
                user_id: user_id.to_owned(),
                portfolio: portfolio.to_owned(),
            },
        );

        UserPortfolio {
            id,
            user_id: *user_id,
            portfolio: portfolio.clone(),
        }
    }

    fn find_by_id(&self, id: &i32) -> UserPortfolio {
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

    fn delete_by_id(&self, id: &i32) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::{PortfolioRepoInMemory, PortfolioRepository};
    use crate::app::Portfolio;

    fn some_portfolio() -> Portfolio {
        Portfolio::from([
            ("A".to_string(), 75_000.0),
            ("B".to_string(), 100_000.0),
            ("C".to_string(), 125_000.0),
        ])
    }

    #[test]
    fn create_and_find_portfolio() {
        let port_repo = PortfolioRepoInMemory::new();
        let some_port = some_portfolio();

        let port_1 = port_repo.create(&1, &some_port);
        let port_2 = port_repo.create(&1, &some_port);

        assert_ne!(port_1, port_2);
    }
}
