use std::{cell::RefCell, collections::HashMap};

use crate::Portfolio;

#[derive(Debug, PartialEq)]
struct UserPortfolio {
    id: i32,
    user_id: i32,
    portfolio: Portfolio,
}

trait PortfolioRepository {
    fn create(&self, user_id: &i32, portfolio: &Portfolio) -> UserPortfolio;

    fn find_by_id(&self, id: &i32) -> UserPortfolio;
    fn find_by_user(&self, user_id: &i32) -> Vec<UserPortfolio>;

    fn delete_by_id(&self, id: &i32);
}

struct PortfolioRepoInMemory {
    portfolios: RefCell<HashMap<i32, Portfolio>>,
    id_counter: RefCell<i32>,
}

impl PortfolioRepoInMemory {
    fn new() -> PortfolioRepoInMemory {
        PortfolioRepoInMemory {
            portfolios: RefCell::new(HashMap::new()),
            id_counter: RefCell::new(0),
        }
    }

    fn next_id(&self) -> i32 {
        let next = *self.id_counter.borrow() + 1;
        *self.id_counter.borrow_mut() = next;

        next
    }
}

impl PortfolioRepository for PortfolioRepoInMemory {
    fn create(&self, user_id: &i32, portfolio: &Portfolio) -> UserPortfolio {
        let id = self.next_id();
        self.portfolios.borrow_mut().insert(id, portfolio.clone());

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
        todo!()
    }

    fn delete_by_id(&self, id: &i32) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::{PortfolioRepoInMemory, PortfolioRepository};
    use crate::Portfolio;

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
