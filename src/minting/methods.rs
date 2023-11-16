use near_sdk::{AccountId, env};
use crate::{Contract, utils};
use crate::metadata::types::{Token, TokenMetadata};
use crate::tokens::traits::NFTToken;
use crate::utils::TokenId;

impl Contract {
    pub fn mint_nft(&mut self, token_id: TokenId, metadata: TokenMetadata, receiver_id: AccountId) {
        let initial_storage_usage = env::storage_usage();

        assert_eq!(self.owner_id, env::predecessor_account_id(), "mint function can only be called by owner");
        assert!(self.tokens_by_id.get(&token_id).is_none(), "token with this id already minted");

        let token = Token {
            owner_id: receiver_id.clone(),
        };
        self.tokens_by_id.insert(&token_id, &token);
        self.increment_owner_tokens(token_id.clone(), receiver_id.clone());

        utils::refund_deposit(env::storage_usage() - initial_storage_usage);
    }
}