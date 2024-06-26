
use anchor_lang::prelude::*;
use anchor_spl::
    token::{
        self, MintTo, Mint, TokenAccount, Token
    }
;

#[derive(Accounts)]
pub struct PrintGems<'info> {
    #[account(
        init_if_needed,
        payer = payer,
        seeds = [b"gems".as_ref(), payer.key().as_ref()], // Ensuring uniqueness per user
        token::mint = mint,
        token::authority = mint,
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

/// Checks if the given data slice is entirely zeroed out.
fn is_zeroed(data: &[u8]) -> bool {
    data.iter().all(|&byte| byte == 0)
}

pub fn handler(ctx: Context<PrintGems>, amount: u64) -> Result<()> {    

    // TODO: Consider refactoring initialization into a discrete instruction
    // to avoid excessive CU usage.

    // Initialize the token account with the program's mint PDA as authority
    if is_zeroed(&ctx.accounts.dst_ata.to_account_info().data.borrow()) {

        let cpi_accounts = token::InitializeAccount {
            account: ctx.accounts.dst_ata.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            authority: ctx.accounts.mint.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::initialize_account(cpi_ctx)?;
    }

    let seeds = &["mint".as_bytes(), "gems".as_bytes(), &[ctx.bumps.mint]];
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