use super::{repository::PriceRepoType, repository_in_memory::RepoInMemory};
use crate::utils::ChainingExt;
use std::{collections::HashMap, sync::Mutex};

pub struct RepositoryBuilder;

impl RepositoryBuilder {
    pub fn in_memory() -> PriceRepoType {
        RepoInMemory {
            data: Mutex::new(HashMap::new()),
        }
        .pipe(Box::new)
    }
}
