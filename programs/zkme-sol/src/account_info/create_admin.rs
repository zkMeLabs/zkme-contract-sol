use crate::prelude::*;

#[derive(Accounts)]
pub struct CreateAdmin<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init,
        seeds = [Admin::SEEDS.as_bytes(), authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + Admin::LEN,
    )]
    pub admin: Account<'info, Admin>,

    pub system_program: Program<'info, System>,
}
