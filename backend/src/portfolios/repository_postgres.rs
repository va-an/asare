use async_trait::async_trait;
use serde_json::json;
use sqlx::{postgres::PgRow, PgPool, Row};

use super::{repository::PortfolioRepository, service::UserPortfolio};

pub struct PortfolioRepoPostgres {
    pool: PgPool,
}

impl PortfolioRepoPostgres {
    pub fn new(pool: PgPool) -> Self {
        PortfolioRepoPostgres { pool }
    }
}

#[async_trait]
impl PortfolioRepository for PortfolioRepoPostgres {
    async fn create(&self, portfolio: &UserPortfolio) -> UserPortfolio {
        sqlx::query(
            "
            INSERT INTO portfolios (user_id, portfolio) 
            VALUES ($1, $2) 
            RETURNING *;",
        )
        .bind(portfolio.user_id)
        .bind(json!(portfolio.portfolio))
        .map(|row: PgRow| UserPortfolio {
            id: row.get("id"),
            user_id: row.get("user_id"),
            portfolio: serde_json::from_str(row.get("portfolio")).unwrap(),
        })
        .fetch_one(&self.pool)
        .await
        .unwrap()
    }

    async fn find_by_id(&self, id: &i32) -> UserPortfolio {
        sqlx::query(
            "
            SELECT * FROM portfolios 
            WHERE id = $1",
        )
        .bind(id)
        .map(|row: PgRow| UserPortfolio {
            id: row.get("id"),
            user_id: row.get("user_id"),
            portfolio: serde_json::from_str(row.get("portfolio")).unwrap(),
        })
        .fetch_one(&self.pool)
        .await
        .unwrap()
    }

    async fn find_by_user(&self, user_id: &i32) -> Vec<UserPortfolio> {
        sqlx::query(
            "
            SELECT * FROM portfolios 
            WHERE user_id = $1",
        )
        .bind(user_id)
        .map(|row: PgRow| UserPortfolio {
            id: row.get("id"),
            user_id: row.get("user_id"),
            portfolio: serde_json::from_str(row.get("portfolio")).unwrap(),
        })
        .fetch_all(&self.pool)
        .await
        .unwrap()
    }

    async fn delete_by_id(&self, id: &i32) {
        sqlx::query(
            "
            DELETE FROM portfolios 
            WHERE id = $1",
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .unwrap();
    }
}
