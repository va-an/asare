use super::{finance_api::FinanceApiType, finance_api_random::FinanceApiRandom};

pub struct FinanceApiBuilder;

impl FinanceApiBuilder {
    pub fn random() -> FinanceApiType {
        Box::new(FinanceApiRandom {})
    }
}
