use sqlx::PgPool;

use super::repository::UserRepository;

pub struct UserRepoPostgres {
    pool: PgPool,
}

impl UserRepoPostgres {
    pub fn new(pool: PgPool) -> Self {
        UserRepoPostgres { pool }
    }
}

impl UserRepository for UserRepoPostgres {
    fn create(
        &self,
        username: &str,
        password: &str,
        api_key: &str,
    ) -> Result<domain::users::User, String> {
        todo!()
    }

    fn delete(&self, username: &str) {
        todo!()
    }

    fn find_all(&self) -> Vec<domain::users::User> {
        todo!()
    }

    fn find_by_api_key(&self, api_key: &str) -> Option<domain::users::User> {
        todo!()
    }

    fn find_all_usernames(&self) -> std::collections::HashSet<String> {
        todo!()
    }
}
