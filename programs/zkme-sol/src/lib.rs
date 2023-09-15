#![allow(clippy::result_large_err)]
mod account_info;
mod error;
mod event;
mod state;

mod prelude {
    pub use anchor_lang::prelude::*;
    pub use anchor_lang::solana_program::program::{invoke, invoke_signed};
    pub use anchor_spl::{associated_token, token_2022};

    pub(crate) use crate::account_info::*;
    pub(crate) use crate::error::*;
    pub(crate) use crate::state::*;
}

use prelude::*;
use std::ops::Not;

declare_id!("iK51qVs6Q1XMfDRJbSExmnVSeLtcacCCLeHStAnWuyq");

#[program]
pub mod zkme_sol {
    use super::*;

    pub fn create_admin(ctx: Context<CreateAdmin>) -> Result<()> {
        let admin = &mut ctx.accounts.admin;

        admin
            .add_operator(ctx.accounts.authority.key())
            .ok_or(ZkmeError::AddOperatorFailed)?;

        Ok(())
    }

    pub fn approve_operator(ctx: Context<ApproveOperator>) -> Result<()> {
        let admin = &mut ctx.accounts.admin;

        admin
            .add_operator(ctx.accounts.new_operator.key())
            .ok_or(ZkmeError::AddOperatorFailed)?;

        Ok(())
    }

    pub fn revoke_operator(ctx: Context<RevokeOperator>) -> Result<()> {
        let admin = &mut ctx.accounts.admin;

        if admin.remove_operator(&ctx.accounts.operator.key()).not() {
            return Err(error!(ZkmeError::InvalidOperator));
        }

        Ok(())
    }

    pub fn create_conf(
        ctx: Context<CreateCooperatorConf>,
        data: String,
        valid_questions: Vec<String>,
    ) -> Result<()> {
        let conf = &mut ctx.accounts.co_conf;

        conf.owner = ctx.accounts.cooperator.key();
        conf.approved_user = 0u64;
        conf.approved_lite_user = 0u64;
        conf.approved = 0u64;
        conf.approved_lite = 0u64;
        conf.data = data;
        conf.save_valid_questions(valid_questions);

        Ok(())
    }

    pub fn update_conf(
        ctx: Context<UpdateCooperatorConf>,
        data: String,
        valid_questions: Vec<String>,
    ) -> Result<()> {
        let conf = &mut ctx.accounts.co_conf;

        conf.data = data;
        conf.save_valid_questions(valid_questions);

        Ok(())
    }

    pub fn mint_sbt(ctx: Context<MintSBT>) -> Result<()> {
        let admin = &mut ctx.accounts.admin;
        admin.next_counter += 1;

        ctx.accounts.invoke_signed_init_untransferable_mint()?;
        ctx.accounts.invoke_signed_init_mint2()?;

        ctx.accounts.invoke_signed_init_immutable_owner()?;
        token_2022::initialize_account3(ctx.accounts.as_init_account3_context())?;

        token_2022::mint_to(ctx.accounts.as_mint_to_context(), 1)?;

        Ok(())
    }

    pub fn create_kyc(
        ctx: Context<CreateKyc>,
        threshold_key: String,
        expiration_date: u64,
        data: String,
        questions: Vec<String>,
    ) -> Result<()> {
        let kyc_acc = &mut ctx.accounts.kyc;

        kyc_acc.status = KycStatus::Initialized;
        kyc_acc.owner = ctx.accounts.user.key();
        kyc_acc.token_account = ctx.accounts.token_account.key();

        kyc_acc.update(threshold_key, expiration_date, data, questions);

        Ok(())
    }

    pub fn update_kyc(
        ctx: Context<UpdateKyc>,
        threshold_key: String,
        expiration_date: u64,
        data: String,
        questions: Vec<String>,
    ) -> Result<()> {
        let kyc_acc = &mut ctx.accounts.kyc;

        kyc_acc.update(threshold_key, expiration_date, data, questions);
        Ok(())
    }

    pub fn approve_sbt(ctx: Context<ApproveSbt>, threshold_key: String) -> Result<()> {
        let co_conf = &mut ctx.accounts.co_conf;
        co_conf.approved_user += 1;
        co_conf.approved += 1;

        let kyc_full = &mut ctx.accounts.kyc_full;
        let kyc = &ctx.accounts.kyc;

        kyc_full.status = KycStatus::Approved;
        kyc_full.owner = ctx.accounts.co_conf.key();
        kyc_full.token_account = ctx.accounts.token_account.key();
        kyc_full.threshold_key = threshold_key;
        // copy last 3 fields from user kyc account
        kyc_full.validity = kyc.validity;
        kyc_full.data = kyc.data.clone();
        kyc_full.questions = kyc.questions.clone();

        Ok(())
    }

    pub fn approve_sbt_lite(ctx: Context<ApproveSbtLite>, threshold_key: String) -> Result<()> {
        let co_conf = &mut ctx.accounts.co_conf;
        co_conf.approved_lite_user += 1;
        co_conf.approved_lite += 1;

        let kyc_lite = &mut ctx.accounts.kyc_lite;

        kyc_lite.status = KycStatus::Approved;
        kyc_lite.threshold_key = threshold_key;
        kyc_lite.owner = ctx.accounts.co_conf.key();
        kyc_lite.token_account = ctx.accounts.token_account.key();
        kyc_lite.kyc = ctx.accounts.kyc.key();
        Ok(())
    }

    pub fn revoke_sbt(ctx: Context<RevokeSbt>) -> Result<()> {
        let co_conf = &mut ctx.accounts.co_conf;
        co_conf.approved -= 1;

        let kyc_full = &mut ctx.accounts.kyc_full;

        kyc_full.status = KycStatus::Revoked;

        Ok(())
    }

    pub fn revoke_sbt_lite(ctx: Context<RevokeSbtLite>) -> Result<()> {
        let co_conf = &mut ctx.accounts.co_conf;
        co_conf.approved_lite -= 1;

        let kyc_lite = &mut ctx.accounts.kyc_lite;

        kyc_lite.status = KycStatus::Revoked;

        Ok(())
    }

    pub fn burn_sbt(ctx: Context<BurnSbt>) -> Result<()> {
        ctx.accounts.invoke_signed_burn()?;

        Ok(())
    }
}
