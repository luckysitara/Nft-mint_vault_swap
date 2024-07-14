use anchor_lang::prelude::*;
use mpl_core::instructions::CreateV1CpiBuilder;

use crate::{
    error::CreateErrorCode,
    state::CollectionData,
    Core,
    MintFromColParams,
};

/// Context for minting an MPL Core asset from a collection.
#[derive(Accounts)]
#[instruction(params: MintFromColParams)]
pub struct MintFromCollectionContext<'info> {
    pub payer: Signer<'info>,

    #[account(mut, signer)]
    pub asset: UncheckedAccount<'info>,

    #[account(mut, address = collection_data.collection @ CreateErrorCode::PubkeyMismatch)]
    pub collection: UncheckedAccount<'info>,

    #[account(mut)]
    pub collection_data: Account<'info, CollectionData>,

    pub core_program: Program<'info, Core>,

    pub system_program: Program<'info, System>,
}

impl MintFromCollectionContext<'_> {
    /// Validation helper for the minting context.
    pub fn validate(&self) -> Result<()> {
        // Check if collection has items available to mint.
        if self.collection_data.items_available == 0 {
            return Err(error!(CreateErrorCode::CollectionMintedOut));
        }
        Ok(())
    }

    /// CPI into mpl_core program and mint the asset.
    #[access_control(ctx.accounts.validate())]
    pub fn mint_from_collection(
        ctx: Context<Self>,
        params: MintFromColParams,
    ) -> Result<()> {
        let collection_data = &mut ctx.accounts.collection_data;

        collection_data.items_available -= 1;

        CreateV1CpiBuilder::new(&ctx.accounts.core_program)
            .asset(&ctx.accounts.asset)
            .collection(Some(&ctx.accounts.collection))
            .authority(Some(&ctx.accounts.payer))
            .payer(&ctx.accounts.payer)
            .owner(Some(&ctx.accounts.payer))
            .system_program(&ctx.accounts.system_program)
            .name(params.name)
            .uri(params.uri)
            .invoke()?;

        Ok(())
    }
}
