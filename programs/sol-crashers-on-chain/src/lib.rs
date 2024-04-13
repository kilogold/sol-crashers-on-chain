use anchor_lang::prelude::*;

mod error;
mod instructions;

use instructions::*;
use error::ErrorCode;


declare_id!("4LaDUjM73BdtSxjAH15rJmGv4aLYGHSMRtXDK6BWhhyq");

#[program]
pub mod sol_crashers_on_chain {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> { 
        initialize::handler(_ctx)
    }

    pub fn print_currency(ctx: Context<PrintCurrency>, amount: u64, kind: Currency) -> Result<()> {
        require!(kind == Currency::Gold, ErrorCode::CustomError);
        print_gold::handler(ctx, amount)
    }
}