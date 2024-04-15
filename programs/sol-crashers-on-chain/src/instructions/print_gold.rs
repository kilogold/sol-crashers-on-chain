
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::*, token::{
        self, MintTo, Mint, TokenAccount, Token
    }
};

#[derive(Accounts)]
pub struct PrintGold<'info> {
    #[account(
        init_if_needed,
        payer = payer,
        token::mint = mint,
        token::authority = mint,
        seeds = [b"gold".as_ref(), payer.key().as_ref()],
        bump
    )]
    pub dst_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"mint", b"gold"],
        bump,
    )]
    pub mint: Account<'info, Mint>,
    
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

/// Checks if the given data slice is entirely zeroed out.
fn is_zeroed(data: &[u8]) -> bool {
    data.iter().all(|&byte| byte == 0)
}

pub fn handler(ctx: Context<PrintGold>, amount: u64) -> Result<()> {

    // TODO: Consider refactoring initialization into a discrete instruction
    // to avoid excessive CU usage.

    // Initialize the token account with the program's mint PDA as authority
    if is_zeroed(&ctx.accounts.dst_ata.to_account_info().data.borrow()) {
        let cpi_accounts = token::InitializeAccount {
            account: ctx.accounts.dst_ata.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };
        let cpi_program = ctx.accounts.associated_token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::initialize_account(cpi_ctx)?;
    }

    let seeds = &["mint".as_bytes(), "gold".as_bytes(), &[ctx.bumps.mint]];
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