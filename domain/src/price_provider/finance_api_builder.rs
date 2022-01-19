use super::{finance_api::FinanceApiType, finance_api_mock::FinanceApiMock};

pub struct FinanceApiBuilder;

impl FinanceApiBuilder {
    pub fn mock() -> FinanceApiType {
        Box::new(FinanceApiMock {})
    }
}
