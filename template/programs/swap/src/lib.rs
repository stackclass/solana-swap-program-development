use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111111");

#[program]
pub mod swap {
    use super::*;

    pub fn make_offer(
        ctx: Context<MakeOffer>,
        id: u64,
        token_a_offered_amount: u64,
        token_b_wanted_amount: u64,
    ) -> Result<()> {
        // Students will implement this function
        Ok(())
    }

    pub fn take_offer(ctx: Context<TakeOffer>) -> Result<()> {
        // Students will implement this function
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct MakeOffer<'info> {
    pub maker: Signer<'info>,
}

#[derive(Accounts)]
pub struct TakeOffer<'info> {
    pub taker: Signer<'info>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sample() {
        assert_eq!(2 + 2, 4);
    }
}
