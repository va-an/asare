use domain::utils::ChainingExt;

use super::{
    repository::UserRepo, repository_in_memory::UserRepoInMemory, repository_pickle::UserRepoPickle,
};

pub struct UserRepositoryBuilder;

impl UserRepositoryBuilder {
    #[allow(dead_code)]
    pub fn in_memory() -> UserRepo {
        UserRepoInMemory::new().pipe(Box::new)
    }

    #[allow(dead_code)]
    pub fn pickle() -> UserRepo {
        UserRepoPickle::new().pipe(Box::new)
    }
}
