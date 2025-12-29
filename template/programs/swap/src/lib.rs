pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("DAehvmx2vZoWCJi7Qo3Y4YF5vrEWHQRJ288kqKwDy5DV");

#[program]
pub mod swap {
    use super::*;

    pub fn make_offer(
        context: Context<MakeOffer>,
        id: u64,
        token_a_offered_amount: u64,
        token_b_wanted_amount: u64,
    ) -> Result<()> {
        // Students will implement this function
        Ok(())
    }

    pub fn take_offer(context: Context<TakeOffer>) -> Result<()> {
        // Students will implement this function
        Ok(())
    }
}