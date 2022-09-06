use std::{collections::HashSet, iter::FromIterator};

use async_trait::async_trait;
use domain::users::User;
use sqlx::{postgres::PgRow, PgPool, Row};

use super::repository::UserRepository;

pub struct UserRepoPostgres {
    pool: PgPool,
}

impl UserRepoPostgres {
    pub fn new(pool: PgPool) -> Self {
        UserRepoPostgres { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepoPostgres {
    async fn create(&self, username: &str, password: &str, api_key: &str) -> Result<User, String> {
        sqlx::query(
            "INSERT INTO users (username, password, api_key) 
            VALUES ($1, $2, $3) 
            RETURNING *;",
        )
        .bind(username)
        .bind(password)
        .bind(api_key)
        .map(|row: PgRow| User {
            id: row.get("id"),
            username: row.get("username"),
            password: row.get("password"),
            api_key: row.get("api_key"),
        })
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn delete(&self, username: &str) {
        sqlx::query(
            "DELETE FROM users 
            WHERE username = $1;",
        )
        .bind(username)
        .execute(&self.pool)
        .await
        .unwrap();
    }

    async fn find_all(&self) -> Vec<User> {
        sqlx::query_as::<_, User>("SELECT * FROM users;")
            .fetch_all(&self.pool)
            .await
            .unwrap()
    }

    async fn find_by_api_key(&self, api_key: &str) -> Option<User> {
        sqlx::query_as::<_, User>(
            "SELECT * FROM users 
            WHERE api_key = $1",
        )
        .bind(api_key)
        .fetch_optional(&self.pool)
        .await
        .unwrap()
    }

    async fn find_all_usernames(&self) -> std::collections::HashSet<String> {
        let usernames = sqlx::query("SELECT username FROM users;")
            .map(|row: PgRow| {
                let username: String = row.get("username");
                username
            })
            .fetch_all(&self.pool)
            .await
            .unwrap();

        HashSet::from_iter(usernames)
    }
}
