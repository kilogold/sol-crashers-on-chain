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
        mint::authority = mint_gold, 
        seeds = [b"mint", b"gold"], 
        bump
    )]
    pub mint_gold: Account<'info, Mint>,

    #[account(
        init, 
        payer = payer, 
        mint::decimals = 0, 
        mint::authority = mint_gems, 
        seeds = [b"mint", b"gems"], 
        bump
    )]
    pub mint_gems: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
        
    pub token_program: Program<'info, Token>,
}

pub fn handler(_ctx: Context<Initialize>) -> Result<()> { 
    msg!("Initializing mint accounts for Gold and Gems.");   
    Ok(())
}