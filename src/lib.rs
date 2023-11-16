use near_sdk::{AccountId, near_bindgen, PanicOnDefault};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};

use crate::metadata::types::{NFTContractMetadata, Token, TokenMetadata};
use crate::utils::TokenId;

mod utils;
mod metadata;
mod minting;
mod tokens;


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner_id: AccountId,
    tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,
    tokens_by_id: LookupMap<TokenId, Token>,
    token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>,
    metadata: LazyOption<NFTContractMetadata>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        Self {
            tokens_per_owner: LookupMap::new(b"o"),
            tokens_by_id: LookupMap::new(b"i"),
            token_metadata_by_id: UnorderedMap::new(
                b"d"
            ),
            owner_id,
            metadata: LazyOption::new(
                b"m",
                Some(&metadata),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::metadata::traits::NFTMetadata;
    use crate::utils::create_contract_metadata;

    use super::*;

    #[test]
    fn it_can_init_with_correct_data() {
        let contract = Contract::new(AccountId::from_str("yash.testnet").unwrap(), create_contract_metadata());

        let metadata = contract.nft_metadata();

        assert_eq!(
            metadata.name,
            "Test Contract".to_string()
        );
        assert_eq!(
            metadata.symbol,
            "TST".to_string()
        );
        assert_eq!(
            metadata.description.unwrap(),
            "A test NFT Contract".to_string()
        );
        assert_eq!(
            metadata.icon_uri,
            "https://icon.uri".to_string()
        );
        assert_eq!(
            metadata.base_uri,
            "https://base.uri".to_string()
        );
    }
}
