use async_trait::async_trait;
use domain::Portfolio;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use super::{repository::PortfolioRepository, repository_postgres::PortfolioRepoPostgres};

#[async_trait]
pub trait Portfolios {
    async fn create(&self, portfolio: UserPortfolio) -> UserPortfolio;
    async fn find_by_user(&self, user_id: i32) -> Vec<UserPortfolio>;
    async fn delete(&self, id: i32, user_id: i32);
}

pub struct PortfoliosImpl {
    port_repo: PortfolioRepoPostgres,
}

impl PortfoliosImpl {
    pub fn new(pool: PgPool) -> PortfoliosImpl {
        let port_repo = PortfolioRepoPostgres::new(pool);

        PortfoliosImpl { port_repo }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, FromRow)]
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

#[async_trait]
impl Portfolios for PortfoliosImpl {
    async fn create(&self, portfolio: UserPortfolio) -> UserPortfolio {
        self.port_repo.create(&portfolio).await
    }

    async fn find_by_user(&self, user_id: i32) -> Vec<UserPortfolio> {
        self.port_repo.find_by_user(user_id).await
    }

    async fn delete(&self, id: i32, user_id: i32) {
        self.port_repo.delete_by_id(id, user_id).await
    }
}
