use domain::utils::ChainingExt;

use super::{mapper::ApiKeyMapperType, mapper_json_file::ApiKeyMapperJsonFile};

pub struct ApiKeyMapperBuilder;

impl ApiKeyMapperBuilder {
    pub fn json_file() -> ApiKeyMapperType {
        ApiKeyMapperJsonFile::new().pipe(Box::new)
    }
}
