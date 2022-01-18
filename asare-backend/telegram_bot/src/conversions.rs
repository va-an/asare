use domain::{
    rebalancer::service::{RebalanceInput, RebalanceOutput},
    Portfolio,
};

pub struct BotController;

impl BotController {
    pub fn from_input(input: &str) -> Result<RebalanceInput, ()> {
        let input_list = input
            .lines()
            .map(|line| line.split(" ").collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>();

        let lines_len_ok = input_list.iter().all(|line| line.len() == 3);
        if !lines_len_ok {
            return Err(());
        }

        let parse_digits_ok = input_list
            .iter()
            .all(|line| line[1].parse::<f32>().is_ok() & line[2].parse::<f32>().is_ok());
        if !parse_digits_ok {
            return Err(());
        }

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

        Ok(RebalanceInput {
            current_portfolio,
            required_allocation,
        })
    }

    // TODO: 2 digits after point (ex 123.46 instead of 123.4567)
    pub fn from_output(output: &RebalanceOutput) -> String {
        fn show_allocation(p: &Portfolio) -> String {
            p.iter().map(|(k, v)| format!("{}:\t{}%\n", k, v)).collect()
        }

        fn port_as_str(p: &Portfolio) -> String {
            p.iter().map(|(k, v)| format!("{}:\t{}\n", k, v)).collect()
        }

        format!(
            "current allocation:\n{}\nrequired operations:\n{}",
            show_allocation(&output.current_allocation),
            port_as_str(&output.required_operations)
        )
    }
}
