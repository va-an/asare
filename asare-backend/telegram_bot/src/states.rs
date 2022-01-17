use derive_more::From;
use teloxide::macros::Transition;

use self::main_lupa::MainLupaState;

pub mod main_lupa;
pub mod rebalance;

#[derive(Clone)]
pub struct RebalanceByAmountState;

#[derive(Clone)]
pub struct RebalanceByPriceState;

#[derive(Transition, Clone, From)]
pub enum RebalanceDialogue {
    MainLupa(MainLupaState),
    RebalanceByAmount(RebalanceByAmountState),
    RebalanceByPrice(RebalanceByPriceState),
}

impl Default for RebalanceDialogue {
    fn default() -> Self {
        Self::MainLupa(MainLupaState)
    }
}
