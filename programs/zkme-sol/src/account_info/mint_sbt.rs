use crate::token_2022::Token2022;
use crate::associated_token::AssociatedToken;
use crate::prelude::*;

#[derive(Accounts)]
pub struct MintSBT<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut,
        constraint = admin.is_operator(&authority.key()) @ ZkmeError::InvalidOperator
    )]
    pub admin: Account<'info, Admin>,

    #[account(mut)]
    /// CHECK: user account
    pub user: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: [MINT_V1_SEEDS, authority.key().as_ref()]
    pub mint: UncheckedAccount<'info>,

    /// CHECK: Associated token account with token_2022
     #[account(mut)]
    pub token_account: UncheckedAccount<'info>,

    pub token_program: Program<'info, token_2022::Token2022>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    pub system_program: Program<'info, System>,
}

