use serde::Deserialize;

use crate::app::Portfolio;

use super::routes::RebalanceOutput;

#[derive(Deserialize, Debug)]
pub struct RebalanceInput {
    pub current_portfolio: Portfolio,
    pub required_allocation: Portfolio,
}

pub struct RebalancerImpl;

pub trait Rebalancer {
    fn calc_current_allocation(portfolio: &Portfolio) -> Portfolio;
    fn calc_expected_portfolio(input: &RebalanceInput) -> Portfolio;
    fn calc_purchase(input: &RebalanceInput) -> Portfolio;
    fn rebalance(input: &RebalanceInput) -> RebalanceOutput;
}

impl Rebalancer for RebalancerImpl {
    fn calc_current_allocation(portfolio: &Portfolio) -> Portfolio {
        let sum: f32 = portfolio.values().sum();

        portfolio
            .iter()
            .map(|(ticker, value)| (ticker.clone(), value / sum * 100.0))
            .collect()
    }

    fn calc_expected_portfolio(input: &RebalanceInput) -> Portfolio {
        let sum: f32 = input.current_portfolio.values().sum();

        input
            .required_allocation
            .iter()
            .map(|(ticker, value)| (ticker.clone(), value / 100.0 * sum))
            .collect()
    }

    fn calc_purchase(input: &RebalanceInput) -> Portfolio {
        let expected_portfolio = RebalancerImpl::calc_expected_portfolio(input);

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

    fn rebalance(input: &RebalanceInput) -> RebalanceOutput {
        let current_allocation = Self::calc_current_allocation(&input.current_portfolio);
        let required_operations = Self::calc_purchase(&input);

        RebalanceOutput {
            current_allocation,
            required_operations,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

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

    #[test]
    fn calculate_allocation() {
        let result = RebalancerImpl::calc_current_allocation(&current_portfolio());

        let sum: f32 = result.values().sum();
        assert_abs_diff_eq!(sum, 100.0);

        assert_abs_diff_eq!(result["A"], 25.0, epsilon = 0.01);
        assert_abs_diff_eq!(result["B"], 33.33, epsilon = 0.01);
        assert_abs_diff_eq!(result["C"], 41.66, epsilon = 0.01);
    }

    #[test]
    fn calculate_expected_portfolio() {
        let expected_allocation = RebalancerImpl::calc_expected_portfolio(&rebalance_input());

        let sum: f32 = expected_allocation.values().sum();
        assert_abs_diff_eq!(sum, 300_000.0);

        assert_abs_diff_eq!(expected_allocation["A"], 99_000.0, epsilon = 0.01);
        assert_abs_diff_eq!(expected_allocation["B"], 99_000.0, epsilon = 0.01);
        assert_abs_diff_eq!(expected_allocation["C"], 102_000.0, epsilon = 0.01);
    }

    #[test]
    fn calculate_purchase() {
        let current_portfolio = current_portfolio();
        let expected_allocation = RebalancerImpl::calc_expected_portfolio(&rebalance_input());
        let purchases = RebalancerImpl::calc_purchase(&rebalance_input());

        for (ticker, value) in &expected_allocation {
            let current_value = current_portfolio.get(ticker).unwrap();
            let purchase_value = purchases.get(ticker).unwrap();

            let actual = current_value + purchase_value;

            assert_abs_diff_eq!(actual, value, epsilon = 0.01)
        }
    }
}
