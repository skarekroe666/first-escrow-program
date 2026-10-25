use anchor_lang::prelude::*;

#[error_code]
pub enum EscrowError {
    #[msg("The maker account does not match the escrow")]
    InvalidMaker,

    #[msg("Mint A does not match the escrow mint A")]
    InvalidMintA,

    #[msg("Mint B does not match the escrow mint B")]
    InvalidMintB,
}
