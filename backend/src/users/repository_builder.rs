use domain::utils::ChainingExt;

use super::{repository::UserRepo, repository_in_memory::UserRepoInMemory};
use std::{collections::HashMap, sync::Mutex};

pub struct UserRepositoryBuilder;

impl UserRepositoryBuilder {
    pub fn in_memory() -> UserRepo {
        UserRepoInMemory {
            users: Mutex::new(HashMap::new()),
            id_counter: Mutex::new(0),
        }
        .pipe(Box::new)
    }
}
