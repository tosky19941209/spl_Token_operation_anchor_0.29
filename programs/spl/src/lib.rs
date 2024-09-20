use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
declare_id!("5vmTot92m5oq6aSkcRNGCvbJTFjhdkB68JUeb59MRw1X");

#[program]
pub mod spl {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
