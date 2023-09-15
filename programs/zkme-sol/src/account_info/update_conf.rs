use crate::prelude::*;

#[derive(Accounts)]
pub struct UpdateCooperatorConf<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        constraint = admin.is_operator(&authority.key()) @ ZkmeError::InvalidOperator
    )]
    pub admin: Account<'info, Admin>,

    /// CHECK:
    pub cooperator: UncheckedAccount<'info>,

    #[account(mut,
        constraint = co_conf.owner == cooperator.key() @ ZkmeError::InvalidCooperator
    )]
    pub co_conf: Account<'info, CooperatorConf>,

    pub system_program: Program<'info, System>,
}
