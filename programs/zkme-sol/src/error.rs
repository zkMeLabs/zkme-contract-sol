use crate::prelude::*;

#[error_code]
pub enum ZkmeError {
    #[msg("Add operator failed")]
    AddOperatorFailed,

    #[msg("Invalid admin account address")]
    InvalidAdminAccount,

    #[msg("Invalid admin seeds")]
    InvalidAdminSeeds,

    #[msg("Invalid operator")]
    InvalidOperator,

    #[msg("Invalid Cooperator")]
    InvalidCooperator,

    #[msg("Invalid approval owner")]
    InvalidApprovalOwner,

    #[msg("Mint failed")]
    MintFailed,

    #[msg("Transfer failed")]
    TransferFailed,

    #[msg("Invalid associated token account")]
    InvalidAssociatedTokenAccount,
}
