use domain::{
    rebalancer::service::{RebalanceInput, RebalanceOutput},
    Portfolio,
};

pub struct BotController;

// FIXME: input validation
impl BotController {
    pub fn from_input(input: &str) -> RebalanceInput {
        let input_list = input
            .lines()
            .map(|line| line.split(" ").collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>();

        let current_portfolio = input_list
            .iter()
            .map(|line| {
                let (ticker, amount) = (line[0], line[1]);
                let amount: f32 = amount.parse().unwrap();

                (ticker.to_owned(), amount)
            })
            .collect::<Portfolio>();

        let required_allocation = input_list
            .iter()
            .map(|line| {
                let (ticker, amount) = (line[0], line[2]);
                let amount: f32 = amount.parse().unwrap();

                (ticker.to_owned(), amount)
            })
            .collect::<Portfolio>();

        RebalanceInput {
            current_portfolio,
            required_allocation,
        }
    }

    pub fn from_output(output: &RebalanceOutput) -> String {
        fn port_as_str(p: &Portfolio) -> String {
            p.iter().map(|(k, v)| format!("{}:\t{}\n", k, v)).collect()
        }

        format!(
            "current allocation:\n{}\nrequired operations:\n{}",
            port_as_str(&output.current_allocation),
            port_as_str(&output.required_operations)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::BotController;
    use crate::states::main_lupa::EXAMPLE_INPUT;

    #[test]
    fn from_input_test() {
        let input = EXAMPLE_INPUT;

        let result = BotController::from_input(input);

        todo!()
    }
}
