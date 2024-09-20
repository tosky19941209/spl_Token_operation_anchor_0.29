use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
declare_id!("9Bq9HL9iDuVywY7HdEiLPj1QWsM65VJv9DdtxmMwT9jL");

#[program]
pub mod spl {
    use super::*;
    use anchor_lang::solana_program::{rent, system_program};
    use anchor_spl::{
        associated_token::{self, Create},
        token::{mint_to, InitializeMint, MintTo},
    };

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn create_token(_ctx: Context<CreateToken>, decimals: u8, amount: u64) -> Result<()> {
        use super::*;
        use anchor_lang::system_program;
        use anchor_spl::associated_token::AssociatedToken;
        use anchor_spl::token::{initialize_mint, mint_to, InitializeMint, Mint, MintTo};

        system_program::create_account(
            CpiContext::new(
                _ctx.accounts.system_program.to_account_info(),
                system_program::CreateAccount {
                    from: _ctx.accounts.signer.to_account_info(),
                    to: _ctx.accounts.mint_token.to_account_info(),
                },
            ),
            10_000_000,
            82,
            _ctx.accounts.token_program.key,
        )?;

        initialize_mint(
            CpiContext::new(
                _ctx.accounts.token_program.to_account_info(),
                InitializeMint {
                    mint: _ctx.accounts.mint_token.to_account_info(),
                    rent: _ctx.accounts.rent.to_account_info(),
                },
            ),
            decimals,
            _ctx.accounts.signer.key,
            Some(_ctx.accounts.signer.key),
        )?;

        associated_token::create(CpiContext::new(
            _ctx.accounts.associate_token_program.to_account_info(),
            associated_token::Create {
                payer: _ctx.accounts.signer.to_account_info(),
                associated_token: _ctx.accounts.associate_token_program.to_account_info(),
                authority: _ctx.accounts.signer.to_account_info(),
                mint: _ctx.accounts.mint_token.to_account_info(),
                system_program: _ctx.accounts.system_program.to_account_info(),
                token_program: _ctx.accounts.token_program.to_account_info(),
            },
        ))?;

        mint_to(
            CpiContext::new(
                _ctx.accounts.token_account.to_account_info(),
                MintTo {
                    authority: _ctx.accounts.signer.to_account_info(),
                    mint: _ctx.accounts.mint_token.to_account_info(),
                    to: _ctx.accounts.token_account.to_account_info(),
                },
            ),
            amount,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct CreateToken<'info> {
    ///CHECK: this is not dangerous
    #[account(mut)]
    pub mint_token: Signer<'info>,
    ///CHECK: this is not dangerous
    #[account(mut)]
    pub signer: Signer<'info>,
    ///CHECK: this is not dangerous
    #[account(mut)]
    pub token_account: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associate_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}
