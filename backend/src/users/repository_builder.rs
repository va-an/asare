use domain::utils::ChainingExt;

use super::{
    repository::UserRepo, repository_in_memory::UserRepoInMemory,
    repository_pickle::UsersRepoPickle,
};

pub struct UserRepositoryBuilder;

impl UserRepositoryBuilder {
    pub fn in_memory() -> UserRepo {
        UserRepoInMemory::new().pipe(Box::new)
    }

    pub fn pickle() -> UserRepo {
        UsersRepoPickle::new().pipe(Box::new)
    }
}
