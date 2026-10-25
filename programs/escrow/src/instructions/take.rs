use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{error::EscrowError, Escrow};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Take<'info> {
    // The account accepting the offer and paying for the transaction.
    #[account(mut)]
    pub taker: Signer<'info>,

    // The wallet that created the escrow and must match the maker in the escrow state.
    #[account(mut)]
    pub maker: SystemAccount<'info>,

    // The escrow PDA that stores the offer details and validates the trade.
    #[account(
        mut,
        close=maker,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
        has_one = maker @ EscrowError::InvalidMaker,
        has_one = mint_a @ EscrowError::InvalidMintA,
        has_one = mint_b @ EscrowError::InvalidMintB,
    )]
    pub escrow: Box<Account<'info, Escrow>>,

    // The mint for the token the maker is offering.
    pub mint_a: Box<InterfaceAccount<'info, Mint>>,

    // The mint for the token the taker must provide.
    pub mint_b: Box<InterfaceAccount<'info, Mint>>,

    // The escrow vault holding the maker's tokens before the trade is completed.
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program,
    )]
    pub vault: Box<InterfaceAccount<'info, TokenAccount>>,

    // The taker's ATA for mint A, created if needed to receive the maker's tokens.
    #[account(
        init_if_needed,
        payer=taker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program,
    )]
    pub taker_ata_a: Box<InterfaceAccount<'info, TokenAccount>>,

    // The taker's ATA for mint B, where their payment tokens are stored.
    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    pub taker_ata_b: Box<InterfaceAccount<'info, TokenAccount>>,

    // The maker's ATA for mint B, created if needed to receive the taker's payment.
    #[account(
        init_if_needed,
        payer=taker,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_ata_b: Box<InterfaceAccount<'info, TokenAccount>>,

    // The program used to create the associated token accounts.
    pub associated_token_program: Program<'info, AssociatedToken>,

    // The token interface program used for account and transfer checks.
    pub token_program: Interface<'info, TokenInterface>,

    // The system program used for creating accounts and handling rent.
    pub system_program: Program<'info, System>,
}
