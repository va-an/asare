use derive_more::From;
use teloxide::macros::Transition;

use self::{main_lupa::MainLupaState, rebalance::RebalanceState};

pub mod main_lupa;
pub mod rebalance;

#[derive(Transition, Clone, From)]
pub enum RebalanceDialogue {
    MainLupa(MainLupaState),
    ReceiveRebalance(RebalanceState),
}

impl Default for RebalanceDialogue {
    fn default() -> Self {
        Self::MainLupa(MainLupaState)
    }
}
