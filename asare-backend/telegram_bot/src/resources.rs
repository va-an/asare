pub static REBALANCE_BY_AMOUNT_COMMAND: &str = "rebalance_by_amount";
pub static REBALANCE_BY_PRICE_COMMAND: &str = "rebalance_by_price";
pub static EXAMPLE_COMMAND: &str = "example";
pub static HELP_COMMAND: &str = "help";
pub static ABOUT_COMMAND: &str = "about";

pub static HELP_MESSAGE: &str = "
Bot for calculation asset allocation rebalance.

Commands:
/rebalance_by_amount - calculate rebalance by ticker and amount
/rebalance_by_price - calculate rebalance by ticker and price
/example - example input message for /rebalance
/help - help
/about - for feature request and bug reports";

pub static EXAMPLE_BY_AMOUNT_MESSAGE: &str = "
For /rebalance_by_amount

Input format:
<ticker> <quantity * price> <requered allocation>

A 75000.0 33
B 100000.0 33
C 125000.0 34
";

pub static EXAMPLE_BY_PRICE_MESSAGE: &str = "
For /rebalance_by_price

Input format:
<ticker> <quantity> <requered allocation>

A 111.0 33
B 56.0 33
C 150.0 34
";

pub static ABOUT_MESSAGE: &str = "
This bot created with love and open source.
Code here - https://github.com/va-anufriev/asare
Feel free create a issues with feature or bugfix requests!";
