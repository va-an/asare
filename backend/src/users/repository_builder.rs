use domain::utils::ChainingExt;
use sqlx::PgPool;

use super::{
    repository::UserRepo, repository_in_memory::UserRepoInMemory,
    repository_postgres::UserRepoPostgres,
};

pub struct UserRepositoryBuilder;

impl UserRepositoryBuilder {
    #[allow(dead_code)]
    pub fn in_memory() -> UserRepo {
        UserRepoInMemory::new().pipe(Box::new)
    }

    #[allow(dead_code)]
    pub fn postgres(pool: PgPool) -> UserRepo {
        UserRepoPostgres::new(pool).pipe(Box::new)
    }
}
