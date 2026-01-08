use anchor_lang::prelude::*;

declare_id!("G8m2Q8kMhkHf3xkVtb9nKfLeiWaSY352QBYsZGH3YfBT");

#[program]
pub mod swap_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
