use anchor_lang::prelude::*;
use mpl_core::instructions::CreateCollectionV1CpiBuilder;

use crate::{
    constants::{SEED_COLLECTION_DATA, SEED_PREFIX},
    state::CollectionData,
    Core,
    CreateCollectionParams,
};

#[derive(Accounts)]
#[instruction(params: CreateCollectionParams)]
pub struct CreateCollectionContext<'info> {
    #[account(mut), signer]
    pub payer: Signer<'info>,

    #[account(mut, signer)]
    pub collection: UncheckedAccount<'info>,

    #[account(
        init,
        payer = payer,
        space = CollectionData::LEN,
        seeds = [SEED_PREFIX, SEED_COLLECTION_DATA, collection.key().as_ref()],
        bump
    )]
    pub collection_data: Account<'info, CollectionData>,

    pub core_program: Program<'info, Core>,

    pub system_program: Program<'info, System>,
}

impl CreateCollectionContext<'_> {
    pub fn validate(&self) -> Result<()> {
        // Optional: Add validation logic if needed
        Ok(())
    }

    pub fn create_collection(
        ctx: Context<CreateCollectionContext>,
        params: CreateCollectionParams,
    ) -> Result<()> {
        let collection_data = &mut ctx.accounts.collection_data;

        *collection_data = CollectionData::new(
            ctx.bumps.collection_data,
            params.items,
            ctx.accounts.payer.key(),
            ctx.accounts.collection.key(),
        );

        CreateCollectionV1CpiBuilder::new(&ctx.accounts.core_program)
            .collection(&ctx.accounts.collection)
            .payer(&ctx.accounts.payer)
            .update_authority(Some(&ctx.accounts.payer))
            .system_program(&ctx.accounts.system_program)
            .name(params.name)
            .uri(params.uri)
            .invoke()?;

        Ok(())
    }
}
