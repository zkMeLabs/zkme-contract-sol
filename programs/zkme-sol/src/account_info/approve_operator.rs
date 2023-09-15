use crate::prelude::*;

#[derive(Accounts)]
pub struct ApproveOperator<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK:
    pub new_operator: UncheckedAccount<'info>,

    #[account(mut,
        constraint = admin.is_operator(&authority.key()) @ ZkmeError::InvalidOperator
    )]
    pub admin: Account<'info, Admin>,
}
