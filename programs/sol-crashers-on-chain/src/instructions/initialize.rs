use anchor_lang::prelude::*;
use anchor_spl::
    token::{
        Mint, Token
    }
;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, 
        payer = payer, 
        mint::decimals = 0, 
        mint::authority = mint, 
        seeds = [b"mint"], 
        bump
    )]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
        
    pub token_program: Program<'info, Token>,
}

pub fn handler(_ctx: Context<Initialize>) -> Result<()> {    
    Ok(())
}