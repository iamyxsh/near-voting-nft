use std::str::FromStr;

use near_sdk::AccountId;

use crate::Contract;
use crate::tokens::traits::NFTToken;
use crate::utils::{create_contract_metadata, create_token_metadata, TokenId};

#[test]
fn it_can_mint_nft() {
    let token_id = TokenId::from_str("1").unwrap();
    let yash_owner = AccountId::from_str("bob.near").unwrap();

    let mut contract = Contract::new(yash_owner.clone(), create_contract_metadata());

    contract.mint_nft(token_id.clone(), create_token_metadata(), yash_owner.clone());
    let token = contract.get_token_by_id(token_id.clone());

    assert_eq!(token.owner_id, yash_owner);
    assert_eq!(token.token_id, token_id);
    assert_eq!(token.metadata, create_token_metadata());
}