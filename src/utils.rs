use std::time::{SystemTime, UNIX_EPOCH};
use env::predecessor_account_id;
use near_sdk::{Balance, env, Promise};

use crate::metadata::types::{NFTContractMetadata, TokenMetadata};

pub type TokenId = String;

pub fn create_contract_metadata() -> NFTContractMetadata {
    NFTContractMetadata {
        name: "Test Contract".to_string(),
        symbol: "TST".to_string(),
        description: Some("A test NFT Contract".to_string()),
        icon_uri: "https://icon.uri".to_string(),
        base_uri: "https://base.uri".to_string(),
    }
}

pub fn create_token_metadata() -> TokenMetadata {
    TokenMetadata {
        title: "Test Token".to_string(),
        description: None,
        media_uri: "https://media.uri".to_string(),
        media_hash: "0xabc".to_string(),
        minted_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
    }
}

pub fn refund_deposit(storage_used: u64) {
    //get how much it would cost to store the information
    let required_cost = env::storage_byte_cost() * Balance::from(storage_used);
    //get the attached deposit
    let attached_deposit = env::attached_deposit();

    //make sure that the attached deposit is greater than or equal to the required cost
    assert!(
        required_cost <= attached_deposit,
        "Must attach {} yoctoNEAR to cover storage",
        required_cost,
    );

    let refund = attached_deposit - required_cost;

    if refund > 1 {
        Promise::new(predecessor_account_id()).transfer(refund);
    }
}