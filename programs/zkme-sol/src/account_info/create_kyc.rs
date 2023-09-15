use crate::prelude::*;

#[derive(Accounts)]
pub struct CreateKyc<'info> {
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

    #[account(init,
        seeds = [Kyc::SEEDS.as_bytes(), user.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + Kyc::LEN,
    )]
    pub kyc: Account<'info, Kyc>,

    pub system_program: Program<'info, System>,
}
