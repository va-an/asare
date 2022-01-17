use super::{api_provider::ApiProviderType, api_provider_mock::ApiProviderMock};

pub struct ApiProviderBuilder;

impl ApiProviderBuilder {
    pub fn mock() -> ApiProviderType {
        Box::new(ApiProviderMock {})
    }
}
