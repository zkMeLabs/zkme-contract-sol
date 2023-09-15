use crate::prelude::*;

#[derive(Accounts)]
pub struct RevokeOperator<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK:
    pub operator: UncheckedAccount<'info>,

    #[account(mut,
        constraint = admin.is_operator(&authority.key()) @ ZkmeError::InvalidOperator
    )]
    pub admin: Account<'info, Admin>,
}
