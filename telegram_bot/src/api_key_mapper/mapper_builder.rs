use domain::utils::ChainingExt;

use super::{mapper::ApiKeyMapperType, mapper_pickle::ApiKeyMapperPickleDb};

pub struct ApiKeyMapperBuilder;

impl ApiKeyMapperBuilder {
    pub fn pickle() -> ApiKeyMapperType {
        ApiKeyMapperPickleDb::new().pipe(Box::new)
    }
}
