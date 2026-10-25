pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use instructions::*;
pub use state::*;

declare_id!("3hfSKDN9J9jdybmTZa2644YhDrjZutMiKdMCFAh2Nza7");

#[program]
pub mod escrow {
    use super::*;
}
