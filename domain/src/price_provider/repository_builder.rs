use super::{repository::PriceRepoType, repository_in_memory::RepoInMemory};
use crate::utils::ChainingExt;
use std::{collections::HashMap, sync::Mutex};

pub struct PricesRepoBuilder;

impl PricesRepoBuilder {
    pub fn in_memory() -> PriceRepoType {
        RepoInMemory {
            data: Mutex::new(HashMap::new()),
        }
        .pipe(Box::new)
    }
}
