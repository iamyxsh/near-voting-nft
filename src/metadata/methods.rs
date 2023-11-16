use crate::Contract;
use crate::metadata::traits::NFTMetadata;
use crate::metadata::types::NFTContractMetadata;

impl NFTMetadata for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}