use near_sdk::AccountId;
use near_sdk::collections::UnorderedSet;

use crate::Contract;
use crate::metadata::types::JsonToken;
use crate::tokens::traits::NFTToken;
use crate::utils::TokenId;

impl NFTToken for Contract {
    fn get_token_by_id(&self, token_id: TokenId) -> JsonToken {
        let token = self.tokens_by_id.get(&token_id);
        assert!(token.is_none(), "Token with ID {} does not exists", token_id);

        let token_metadata = self.token_metadata_by_id.get(&token_id);
        assert!(token_metadata.is_none(), "Token with ID {} does not exists", token_id);

        JsonToken {
            token_id: token_id.clone(),
            owner_id: token.unwrap().owner_id,
            metadata: self.token_metadata_by_id.get(&token_id).unwrap(),
        }
    }

    fn increment_owner_tokens(&mut self, token_id: TokenId, account_id: AccountId) {
        let mut tokens_set = self.tokens_per_owner.get(&account_id).unwrap_or_else(|| {
            UnorderedSet::new(
                b"o"
            )
        });

        tokens_set.insert(&token_id);

        self.tokens_per_owner.insert(&account_id, &tokens_set);
    }
}