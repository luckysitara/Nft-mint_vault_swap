use anchor_lang::prelude::*;

#[error_code]
pub enum CreateErrorCode {
    #[msg("There are no more assets that can be minted from collection.")]
    CollectionMintedOut,
    #[msg(" The Received key doesn't match expected key.")]
    PubkeyMismatch,
    #[msg("You don't have  enough SOL to cover rent for locking asset.")]
    InsufficientLamportsForRent,
    #[msg(" You Don't have  enough SOL to purchase asset.")]
    InsufficientLamportsForPurchase,
}
