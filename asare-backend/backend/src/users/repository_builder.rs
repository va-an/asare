use super::{repository::UserReposotory, repository_in_memory::UserRepoInMemory};
use crate::utils::ChainingExt;
use std::{collections::HashMap, sync::Mutex};

pub type UserRepo = Box<dyn UserReposotory + Sync + Send>;

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
