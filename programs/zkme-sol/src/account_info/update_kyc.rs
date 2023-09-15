use crate::prelude::*;

#[derive(Accounts)]
pub struct UpdateKyc<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        constraint = admin.is_operator(&authority.key()) @ ZkmeError::InvalidOperator
    )]
    pub admin: Account<'info, Admin>,

    /// CHECK: must be initialized
    pub token_account: UncheckedAccount<'info>,

    /// CHECK:
    pub user: UncheckedAccount<'info>,

    #[account(mut,
        constraint = kyc.owner == user.key(),
        constraint = kyc.token_account == token_account.key(),
    )]
    pub kyc: Account<'info, Kyc>,
}
