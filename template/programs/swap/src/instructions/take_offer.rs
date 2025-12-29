use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::Offer;

use super::transfer_tokens;

#[derive(Accounts)]
pub struct TakeOffer<'info> {
    // Students will complete this struct
}

pub fn send_wanted_tokens_to_maker(context: &Context<TakeOffer>) -> Result<()> {
    // Students will implement this function
    Ok(())
}

pub fn withdraw_and_close_vault(context: Context<TakeOffer>) -> Result<()> {
    // Students will implement this function
    Ok(())
}