use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::*, token::{
        self, MintTo, Mint, TokenAccount, Token
    }
};

declare_id!("4LaDUjM73BdtSxjAH15rJmGv4aLYGHSMRtXDK6BWhhyq");

#[program]
pub mod sol_crashers_on_chain {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {    
        Ok(())
    }

    pub fn print_gold(ctx: Context<PrintGold>, amount: u64) -> Result<()> {
        let seeds = &["mint".as_bytes(), &[ctx.bumps.mint]];
        let signer = [&seeds[..]];
    
        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(), 
                MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.dst_ata.to_account_info(),
                    authority: ctx.accounts.mint.to_account_info(),
                },
                &signer,
            ), 
            amount
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct PrintGold<'info> {
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer
    )]
    pub dst_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"mint"],
        bump,
    )]
    pub mint: Account<'info, Mint>,
    
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

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