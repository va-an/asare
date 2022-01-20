use super::{finance_api::FinanceApiType, finance_api_mock::FinanceApiMock};

pub struct FinanceApiBuilder;

impl FinanceApiBuilder {
    pub fn random() -> FinanceApiType {
        Box::new(FinanceApiMock {})
    }
}
