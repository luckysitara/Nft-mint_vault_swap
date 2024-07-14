use anchor_lang::prelude::*;
use solana_program::native_token::LAMPORTS_PER_SOL;

use crate::{
    constants::{ADMIN_ADDRESS, SEED_ASSET_MANAGER, SEED_PREFIX},
    AssetManager,
    error::CreateErrorCode,
    Core,
    Protocol,
    SEED_PROTOCOL,
};

/// Context for initializing AssetManager escrow and Protocol account.
#[derive(Accounts)]
pub struct InitContext<'info> {
    #[account(mut, address = ADMIN_ADDRESS @CreateErrorCode::PubkeyMismatch)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = AssetManager::LEN,
        seeds = [SEED_PREFIX, SEED_ASSET_MANAGER],
        bump
    )]
    pub asset_manager: Account<'info, AssetManager>,

    #[account(
        init,
        payer = payer,
        space = Protocol::LEN,
        seeds = [SEED_PREFIX, SEED_PROTOCOL],
        bump
    )]
    pub protocol: Account<'info, Protocol>,

    pub treasury: SystemAccount<'info>,

    pub core_program: Program<'info, Core>,

    pub system_program: Program<'info, System>,
}

impl InitContext<'_> {
    /// Validation helper for the initialization context.
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }

    /// Initializes the Asset Manager escrow account.
    #[access_control(ctx.accounts.validate())]
    pub fn init(ctx: Context<Self>) -> Result<()> {
        msg!("Initialized escrow account");

        let asset_manager = &mut ctx.accounts.asset_manager;
        asset_manager.bump = ctx.bumps.asset_manager;

        let protocol = &mut ctx.accounts.protocol;
        protocol.treasury = ctx.accounts.treasury.key();
        protocol.rent = 1 * LAMPORTS_PER_SOL; // Fixed rental fees

        Ok(())
    }
}
