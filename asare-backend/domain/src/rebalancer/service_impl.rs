use crate::{price_provider::price_provider::PriceProviderType, Portfolio};

use super::service::{RebalanceInput, RebalanceOutput, Rebalancer};

pub struct RebalancerImpl {
    pub price_provider: PriceProviderType,
}

impl Rebalancer for RebalancerImpl {
    fn calc_current_allocation(&self, portfolio: &Portfolio) -> Portfolio {
        let sum: f32 = portfolio.values().sum();

        portfolio
            .iter()
            .map(|(ticker, value)| (ticker.clone(), value / sum * 100.0))
            .collect()
    }

    fn calc_expected_portfolio(&self, input: &RebalanceInput) -> Portfolio {
        let sum: f32 = input.current_portfolio.values().sum();

        input
            .required_allocation
            .iter()
            .map(|(ticker, value)| (ticker.clone(), value / 100.0 * sum))
            .collect()
    }

    fn calc_purchase(&self, input: &RebalanceInput) -> Portfolio {
        let expected_portfolio = self.calc_expected_portfolio(input);

        expected_portfolio
            .iter()
            .map(|(ticker, value)| {
                (
                    ticker.clone(),
                    value - input.current_portfolio.get(ticker).unwrap(),
                )
            })
            .collect()
    }

    fn rebalance(&self, input: &RebalanceInput) -> RebalanceOutput {
        let current_allocation = self.calc_current_allocation(&input.current_portfolio);
        let required_operations = self.calc_purchase(&input);

        RebalanceOutput {
            current_allocation,
            required_operations,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        price_provider::{
            api_provider_builder::ApiProviderBuilder, price_provider_builder::PriceProviderBuilder,
            repository_builder::PricesRepoBuilder,
        },
        rebalancer::{service::RebalancerSvc, service_builder::RebalancerSvcBuilder},
    };

    use super::*;
    use approx::*;

    fn current_portfolio() -> Portfolio {
        Portfolio::from([
            ("A".to_string(), 75_000.0),
            ("B".to_string(), 100_000.0),
            ("C".to_string(), 125_000.0),
        ])
    }

    fn required_allocation() -> HashMap<String, f32> {
        HashMap::from([
            ("A".to_string(), 33.0),
            ("B".to_string(), 33.0),
            ("C".to_string(), 34.0),
        ])
    }

    fn rebalance_input() -> RebalanceInput {
        RebalanceInput {
            current_portfolio: current_portfolio(),
            required_allocation: required_allocation(),
        }
    }

    fn get_svc() -> RebalancerSvc {
        let api_provider = ApiProviderBuilder::mock();
        let prices_repo = PricesRepoBuilder::in_memory();
        let price_provider = PriceProviderBuilder::default(api_provider, prices_repo);
        let svc = RebalancerSvcBuilder::default(price_provider);

        svc
    }

    #[test]
    fn calculate_allocation() {
        let svc = get_svc();

        let result = svc.calc_current_allocation(&current_portfolio());

        let sum: f32 = result.values().sum();
        assert_abs_diff_eq!(sum, 100.0);

        assert_abs_diff_eq!(result["A"], 25.0, epsilon = 0.01);
        assert_abs_diff_eq!(result["B"], 33.33, epsilon = 0.01);
        assert_abs_diff_eq!(result["C"], 41.66, epsilon = 0.01);
    }

    #[test]
    fn calculate_expected_portfolio() {
        let svc = get_svc();

        let expected_allocation = svc.calc_expected_portfolio(&rebalance_input());

        let sum: f32 = expected_allocation.values().sum();
        assert_abs_diff_eq!(sum, 300_000.0);

        assert_abs_diff_eq!(expected_allocation["A"], 99_000.0, epsilon = 0.01);
        assert_abs_diff_eq!(expected_allocation["B"], 99_000.0, epsilon = 0.01);
        assert_abs_diff_eq!(expected_allocation["C"], 102_000.0, epsilon = 0.01);
    }

    #[test]
    fn calculate_purchase() {
        let svc = get_svc();

        let current_portfolio = current_portfolio();
        let expected_allocation = svc.calc_expected_portfolio(&rebalance_input());
        let purchases = svc.calc_purchase(&rebalance_input());

        for (ticker, value) in &expected_allocation {
            let current_value = current_portfolio.get(ticker).unwrap();
            let purchase_value = purchases.get(ticker).unwrap();

            let actual = current_value + purchase_value;

            assert_abs_diff_eq!(actual, value, epsilon = 0.01)
        }
    }
}
