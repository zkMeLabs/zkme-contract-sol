use crate::prelude::*;

#[derive(Accounts)]
pub struct ApproveSbtLite<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub co_conf: Account<'info, CooperatorConf>,

    #[account(init,
        seeds = [KycLite::SEEDS.as_bytes(), &co_conf.approved_lite_user.to_le_bytes(), co_conf.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + KycLite::LEN,
    )]
    pub kyc_lite: Account<'info, KycLite>,

    /// CHECK:
    pub token_account: UncheckedAccount<'info>,

    #[account(
        constraint = kyc.owner == authority.key(),
        constraint = kyc.token_account == token_account.key(),
    )]
    pub kyc: Box<Account<'info, Kyc>>,

    pub system_program: Program<'info, System>,
}
