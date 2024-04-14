use anchor_lang::prelude::*;

mod error;
mod instructions;
mod state;

use instructions::*;


declare_id!("4LaDUjM73BdtSxjAH15rJmGv4aLYGHSMRtXDK6BWhhyq");

#[program]
pub mod sol_crashers_on_chain {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> { 
        initialize::handler(_ctx)
    }

    pub fn print_gold(ctx: Context<PrintGold>, amount: u64) -> Result<()> {
        print_gold::handler(ctx, amount)
    }

    pub fn print_gems(ctx: Context<PrintGems>, amount: u64) -> Result<()> {
        print_gems::handler(ctx, amount)
    }
}