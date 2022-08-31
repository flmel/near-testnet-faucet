use crate::external::*;
use crate::*;

use near_contract_standards::fungible_token::{
    metadata::FungibleTokenMetadata, receiver::FungibleTokenReceiver,
};
use near_sdk::{
    log,
    serde::{Deserialize, Serialize},
    serde_json, Gas, PromiseError, PromiseOrValue,
};

pub const TGAS: u64 = 1_000_000_000_000;

// Message parameters to receive via token function call.
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
#[serde(untagged)]
enum TokenReceiverMessage {
    List { ft_request_allowance: U128 },
}

#[near_bindgen]
#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FTconfig {
    ft_request_allowance: Balance,
    ft_available_balance: Balance,
    ft_metadata: FungibleTokenMetadata,
}

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        // if the message is ""
        if msg.is_empty() {
            match self.ft_faucet.contains_key(&env::predecessor_account_id()) {
                false => {
                    // Token not listed: refund
                    log!("This FT Contract has not been listed yet");
                    return PromiseOrValue::Value(amount);
                }
                true => {
                    // Token listed: update
                    self.ft_faucet
                        .get_mut(&env::predecessor_account_id())
                        .unwrap()
                        .ft_available_balance += amount.0;
                    log!("This FT Contract has been updated");
                    return PromiseOrValue::Value(U128(0));
                }
            }
        }

        // if the message is List add it to the ft_faucet HashMap
        let message = serde_json::from_str::<TokenReceiverMessage>(&msg).expect("WRONG MSG FORMAT");

        match message {
            TokenReceiverMessage::List {
                ft_request_allowance,
            } => {
                // The message matches we do XCC to get the ft_metadata
                // TODO case when FT contract does not implement ft_metadata
                let promise = ft_contract::ext(env::predecessor_account_id())
                    .with_static_gas(Gas(50 * TGAS))
                    .ft_metadata()
                    .then(
                        Self::ext(env::current_account_id())
                            .with_static_gas(Gas(50 * TGAS))
                            .ft_add_token(
                                env::predecessor_account_id(),
                                ft_request_allowance.0,
                                amount.0,
                            ),
                    );
                PromiseOrValue::from(promise)
            }
        }
    }
}

#[near_bindgen]
impl Contract {
    // List new FT in the Faucet
    #[private]
    pub fn ft_add_token(
        &mut self,
        #[callback_result] call_result: Result<FungibleTokenMetadata, PromiseError>,
        ft_account_id: AccountId,
        ft_request_allowance: Balance,
        ft_available_balance: Balance,
    ) -> PromiseOrValue<U128> {
        match call_result {
            Ok(ft_metadata) => {
                // Result is Ok store into ft_faucet HashMap
                self.ft_faucet.insert(
                    ft_account_id,
                    FTconfig {
                        ft_request_allowance,
                        ft_available_balance,
                        ft_metadata,
                    },
                );
                // Log Successful message
                log!("Token added Successfully");
            }
            // Log Error message
            Err(err) => log!("{:#?}", err),
        }
        // TODO add proper docs url
        log!("If you made a mistake or want to know more call view method ft_help on {} or visit URL_HERE ", env::current_account_id());

        PromiseOrValue::Value(U128(0))
    }

    // TODO Change Token
    // TODO De-list Token
    // TODO Get Token INFO
    // TODO List all Tokens
    // TODO Request FT
}
