use crate::prelude::*;

#[derive(Accounts)]
pub struct RevokeSbt<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub co_conf: Account<'info, CooperatorConf>,

    #[account(mut,
        constraint = kyc_full.owner == authority.key() @ ZkmeError::InvalidApprovalOwner
    )]
    pub kyc_full: Account<'info, Kyc>,
}
