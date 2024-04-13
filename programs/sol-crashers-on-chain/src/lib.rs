use anchor_lang::prelude::*;

pub mod instructions;
pub use instructions::*;


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
}