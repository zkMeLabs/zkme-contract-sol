use crate::prelude::*;

#[derive(Accounts)]
pub struct ApproveSbt<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub co_conf: Account<'info, CooperatorConf>,

    #[account(init,
        seeds = [Kyc::SEEDS.as_bytes(), &co_conf.approved_user.to_le_bytes(), co_conf.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + Kyc::LEN,
    )]
    pub kyc_full: Account<'info, Kyc>,

    /// CHECK: Immutable owner token account
    pub token_account: UncheckedAccount<'info>,

    #[account(
        constraint = kyc.owner == authority.key(),
        constraint = kyc.token_account == token_account.key(),
    )]
    pub kyc: Box<Account<'info, Kyc>>,

    pub system_program: Program<'info, System>,
}
