use crate::prelude::*;

#[derive(Accounts)]
pub struct CreateCooperatorConf<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        constraint = admin.is_operator(&authority.key()) @ ZkmeError::InvalidOperator
    )]
    pub admin: Account<'info, Admin>,

    /// CHECK:
    pub cooperator: UncheckedAccount<'info>,

    #[account(init,
        seeds = [CooperatorConf::SEEDS.as_bytes(), cooperator.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + CooperatorConf::LEN,
    )]
    pub co_conf: Account<'info, CooperatorConf>,

    pub system_program: Program<'info, System>,
}
