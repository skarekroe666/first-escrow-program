use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, TransferChecked},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::Escrow;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make<'info> {
    // The maker is the wallet creating the escrow and paying for the account creation.
    #[account(mut)]
    pub maker: Signer<'info>,

    // The escrow PDA stores the deal state, including the maker, mint addresses, and amount.
    #[account(
        init,
        payer = maker,
        space = Escrow::INIT_SPACE + Escrow::DISCRIMINATOR.len(),
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub escrow: Account<'info, Escrow>,

    // The token mint for the asset the maker is offering in the escrow.
    #[account(
        mint::token_program = token_program,
    )]
    pub mint_a: InterfaceAccount<'info, Mint>,

    // The token mint for the asset the maker wants to receive in return.
    #[account(
        mint::token_program = token_program,
    )]
    pub mint_b: InterfaceAccount<'info, Mint>,

    // The maker's ATA for mint A, where the funds are currently held before deposit.
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program,

    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    // The escrow vault ATA that will receive the maker's tokens once the escrow is funded.
    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program,

    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    // The SPL Associated Token Account program used to create the vault ATA.
    pub associated_token_program: Program<'info, AssociatedToken>,

    // The Token Interface program used for token mint and transfer checks.
    pub token_program: Interface<'info, TokenInterface>,

    // The System program needed to fund and create the escrow account and ATA.
    pub system_program: Program<'info, System>,
}

impl<'info> Make<'info> {
    /// Initializes the escrow account with the core parameters for this trade.
    ///
    /// The escrow PDA is seeded with the maker's public key and the instruction seed so that
    /// each maker can create multiple distinct escrow instances without collision. This stores the
    /// actual market terms on-chain for later validation by the taker and settlement logic.
    fn populate_escrow(&mut self, seed: u64, amount: u64, bump: u8) -> Result<()> {
        self.escrow.set_inner(Escrow {
            seed,
            maker: self.maker.key(),
            mint_a: self.mint_a.key(),
            mint_b: self.mint_b.key(),
            receive: amount,
            bump: bump,
        });

        Ok(())
    }

    /// Transfers the maker's tokens into the escrow vault for the agreed amount.
    ///
    /// This is the actual funding step of the offer. The tokens are moved from the maker's ATA for
    /// mint A into the escrow-owned ATA for the same mint, using the token program's checked transfer
    /// semantics to ensure the amount and mint match the account configuration.
    fn deposit_token(&mut self, amount: u64) -> Result<()> {
        transfer_checked(
            CpiContext::new(
                self.token_program.key(),
                TransferChecked {
                    from: self.maker_ata_a.to_account_info(),
                    mint: self.mint_a.to_account_info(),
                    to: self.mint_b.to_account_info(),
                    authority: self.maker.to_account_info(),
                },
            ),
            amount,
            self.mint_a.decimals,
        )?;
        Ok(())
    }
}
