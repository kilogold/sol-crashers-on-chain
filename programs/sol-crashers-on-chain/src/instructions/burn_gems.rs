
use anchor_lang::prelude::*;
use anchor_spl::
    token::{
        self, Mint, TokenAccount, Token, Burn
    }
;

#[derive(Accounts)]
pub struct BurnGems<'info> {
    #[account(
        mut,
        seeds = [b"gems".as_ref(), payer.key().as_ref()],
        bump
    )]
    pub dst_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"mint", b"gems"],
        bump,
    )]
    pub mint: Account<'info, Mint>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<BurnGems>, amount: u64) -> Result<()> {
    let seeds = &["mint".as_bytes(), "gems".as_bytes(), &[ctx.bumps.mint]];
    let signer = [&seeds[..]];

    token::burn(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.mint.to_account_info(),
                from: ctx.accounts.dst_ata.to_account_info(),
                authority: ctx.accounts.mint.to_account_info(),
            },
            &signer,
        ), 
        amount)?;

    Ok(())
}