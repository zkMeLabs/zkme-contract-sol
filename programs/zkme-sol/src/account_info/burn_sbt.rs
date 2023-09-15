use crate::prelude::*;

#[derive(Accounts)]
pub struct BurnSbt<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK:
    pub token_account: UncheckedAccount<'info>,
    /// CHECK:
    pub mint: UncheckedAccount<'info>,

    pub token_program: Program<'info, token_2022::Token2022>,
}

impl<'info> BurnSbt<'info> {
    pub fn invoke_signed_burn(&self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = token_2022::Burn {
            mint: self.mint.to_account_info(),
            from: self.token_account.to_account_info(),
            authority: self.authority.to_account_info(),
        };

        token_2022::burn(
            CpiContext::new(cpi_program, cpi_accounts).with_signer(&[]),
            1,
        )?;

        Ok(())
    }
}
