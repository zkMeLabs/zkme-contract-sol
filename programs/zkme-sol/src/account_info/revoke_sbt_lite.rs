use crate::prelude::*;

#[derive(Accounts)]
pub struct RevokeSbtLite<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub co_conf: Account<'info, CooperatorConf>,

    #[account(mut,
        constraint = kyc_lite.owner == authority.key() @ ZkmeError::InvalidApprovalOwner
    )]
    pub kyc_lite: Account<'info, KycLite>,
}
