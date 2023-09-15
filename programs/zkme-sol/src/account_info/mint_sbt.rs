use crate::prelude::*;

#[derive(Accounts)]
pub struct MintSBT<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut,
        constraint = admin.is_operator(&authority.key()) @ ZkmeError::InvalidOperator
    )]
    pub admin: Account<'info, Admin>,

    /// CHECK: user account
    pub user: UncheckedAccount<'info>,

    /// CHECK: [MINT_V1_SEEDS, authority.key().as_ref()]
    pub mint: UncheckedAccount<'info>,

    /// CHECK: Associated token account with token_2022
    pub token_account: UncheckedAccount<'info>,

    pub token_program: Program<'info, token_2022::Token2022>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> MintSBT<'info> {
    pub fn invoke_signed_init_untransferable_mint(&self) -> Result<()> {
        let inst = token_2022::spl_token_2022::instruction::initialize_non_transferable_mint(
            self.token_program.key,
            self.mint.key,
        )?;
        let admin_seeds = &[
            Admin::SEEDS.as_bytes(),
            &self.admin.next_counter.to_le_bytes(),
            &self.admin.owner.as_ref(),
        ];
        let (admin_pda, bump_seed) = Pubkey::try_find_program_address(&admin_seeds[..], &crate::ID)
            .ok_or(ZkmeError::InvalidAdminSeeds)?;
        require!(
            admin_pda == self.admin.key(),
            ZkmeError::InvalidAdminAccount
        );
        let signers_seeds = &[
            Admin::SEEDS.as_bytes(),
            &self.admin.next_counter.to_le_bytes(),
            &self.admin.owner.as_ref(),
            &[bump_seed],
        ];

        invoke_signed(&inst, &[self.mint.to_account_info()], &[&signers_seeds[..]])?;

        Ok(())
    }

    pub fn invoke_signed_init_mint2(&self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = token_2022::InitializeMint2 {
            mint: self.mint.to_account_info(),
        };

        let admin_seeds = &[
            Admin::SEEDS.as_bytes(),
            &self.admin.next_counter.to_le_bytes(),
            &self.admin.owner.as_ref(),
        ];
        let (_admin_pda, bump_seed) =
            Pubkey::try_find_program_address(&admin_seeds[..], &crate::ID)
                .ok_or(ZkmeError::InvalidAdminSeeds)?;
        let signers_seeds = &[
            Admin::SEEDS.as_bytes(),
            &self.admin.next_counter.to_le_bytes(),
            &self.admin.owner.as_ref(),
            &[bump_seed],
        ];

        token_2022::initialize_mint2(
            CpiContext::new(cpi_program, cpi_accounts).with_signer(&[&signers_seeds[..]]),
            0,
            &self.admin.key(),
            None,
        )?;
        Ok(())
    }

    pub fn invoke_signed_init_immutable_owner(&self) -> Result<()> {
        let token_account_pda = associated_token::get_associated_token_address_with_program_id(
            self.user.key,
            self.mint.key,
            self.token_program.key,
        );
        require!(
            &token_account_pda == self.token_account.key,
            ZkmeError::InvalidAssociatedTokenAccount
        );
        let inst = token_2022::spl_token_2022::instruction::initialize_immutable_owner(
            self.token_program.key,
            self.token_account.key,
        )?;

        invoke_signed(
            &inst,
            &[
                self.token_program.to_account_info(),
                self.token_account.to_account_info(),
            ],
            &[],
        )?;

        Ok(())
    }

    pub fn as_init_account3_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, token_2022::InitializeAccount3<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = token_2022::InitializeAccount3 {
            account: self.token_account.to_account_info(),
            mint: self.mint.to_account_info(),
            authority: self.authority.to_account_info(),
        };

        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn as_mint_to_context(&self) -> CpiContext<'_, '_, '_, 'info, token_2022::MintTo<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = token_2022::MintTo {
            mint: self.mint.to_account_info(),
            to: self.token_account.to_account_info(),
            authority: self.authority.to_account_info(),
        };

        CpiContext::new(cpi_program.clone(), cpi_accounts)
    }
}
