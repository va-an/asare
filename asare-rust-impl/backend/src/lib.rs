mod routes;

use routes::Routes;
use serde_derive::Deserialize;
use std::collections::HashMap;

type Portfolio = HashMap<String, f32>;

pub struct AsareApp {
    port: u16,
}

impl AsareApp {
    pub fn new(port: u16) -> AsareApp {
        AsareApp { port }
    }

    pub async fn run(self) -> std::io::Result<()> {
        Routes::run_http_server(self.port).await
    }
}

#[derive(Deserialize, Debug)]
pub struct RebalanceInput {
    current_portfolio: Portfolio,
    required_allocation: Portfolio,
}

struct RebalancerV1 {}

trait Rebalancer {
    fn calc_current_allocation(portfolio: &Portfolio) -> Portfolio;
    fn calc_expected_portfolio(input: &RebalanceInput) -> Portfolio;
    fn calc_purchase(input: &RebalanceInput) -> Portfolio;
}

impl Rebalancer for RebalancerV1 {
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
        let expected_portfolio = RebalancerV1::calc_expected_portfolio(input);

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
}

#[cfg(test)]
mod tests {
    use crate::*;
    use approx::*;

    // TODO: how store shared objects instead functions?
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
        let result = RebalancerV1::calc_current_allocation(&current_portfolio());

        let sum: f32 = result.values().sum();
        assert_abs_diff_eq!(sum, 100.0);

        let get_and_compare = |key: &str, to_compare: f32| {
            assert_abs_diff_eq!(*result.get(key).unwrap(), to_compare, epsilon = 0.01);
        };

        get_and_compare("A", 25.0);
        get_and_compare("B", 33.33);
        get_and_compare("C", 41.66);
    }

    #[test]
    fn calculate_expected_portfolio() {
        let expected_allocation = RebalancerV1::calc_expected_portfolio(&rebalance_input());

        let sum: f32 = expected_allocation.values().sum();
        assert_abs_diff_eq!(sum, 300_000.0);

        // TODO: duplicated - move to function?
        let get_and_compare = |key: &str, to_compare: f32| {
            assert_abs_diff_eq!(
                *expected_allocation.get(key).unwrap(),
                to_compare,
                epsilon = 0.01
            );
        };

        get_and_compare("A", 99_000.0);
        get_and_compare("B", 99_000.0);
        get_and_compare("C", 102_000.0);
    }

    #[test]
    fn calculate_purchase() {
        let current_portfolio = current_portfolio();
        let expected_allocation = RebalancerV1::calc_expected_portfolio(&rebalance_input());
        let purchases = RebalancerV1::calc_purchase(&rebalance_input());

        for (ticker, value) in &expected_allocation {
            let current_value = current_portfolio.get(ticker).unwrap();
            let purchase_value = purchases.get(ticker).unwrap();

            let actual = current_value + purchase_value;

            assert_abs_diff_eq!(actual, value, epsilon = 0.01)
        }
    }
}
