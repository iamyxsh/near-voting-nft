use crate::metadata::types::NFTContractMetadata;

pub trait NFTMetadata {
    fn nft_metadata(&self) -> NFTContractMetadata;
} 