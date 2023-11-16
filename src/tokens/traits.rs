use near_sdk::AccountId;

use crate::metadata::types::JsonToken;
use crate::utils::TokenId;

pub trait NFTToken {
    fn get_token_by_id(&self, token_id: TokenId) -> JsonToken;
    fn increment_owner_tokens(&mut self, token_id: TokenId, account_id: AccountId);
}