use anchor_lang::prelude::*;

// Modules for organizing code
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

// Re-export modules for external use
pub use constants::*;
pub use instructions::*;
pub use state::*;

// Define the program ID
declare_id!("7B7qYnQAV8SfuxsnkXeWm63H8vYwuKxmWX55avBARUCL");

// Anchor ID implementation for Core
#[derive(Clone)]
pub struct Core;

impl anchor_lang::Id for Core {
    fn id() -> Pubkey {
        mpl_core::ID // Replace with actual ID if necessary
    }
}

// Anchor program module
#[program]
pub mod tims_nft_mint {
    use super::*;

    /// Initializes the protocol configuration and accounts.
    /// This function should be called once to set up the program.
    pub fn init(ctx: Context<InitContext>) -> Result<()> {
        InitContext::init(ctx)
    }

    /// Creates a MPL Core collection.
    /// This function allows creating a new collection within the protocol.
    pub fn create_collection(
        ctx: Context<CreateCollectionContext>,
        params: CreateCollectionParams,
    ) -> Result<()> {
        CreateCollectionContext::create_collection(ctx, params)
    }

    /// Mints a MPL Core asset from a collection.
    /// This function allows creating new assets within an existing collection.
    pub fn mint_asset(ctx: Context<MintFromCollectionContext>, params: MintFromColParams) -> Result<()> {
        MintFromCollectionContext::mint_from_collection(ctx, params)
    }

    /// Locks an asset in a vault.
    /// This function locks an existing asset within a designated vault.
    pub fn lock_in_vault(ctx: Context<LockAssetInVaultContext>) -> Result<()> {
        LockAssetInVaultContext::lock_asset_in_vault(ctx)
    }

    /// Purchases an asset.
    /// This function enables purchasing an asset within the protocol.
    pub fn purchase(ctx: Context<PurchaseContext>) -> Result<()> {
        PurchaseContext::purchase_asset(ctx)
    }
}

