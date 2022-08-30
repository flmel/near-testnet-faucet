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
pub const NO_DEPOSIT: u128 = 0;
pub const XCC_SUCCESS: u64 = 1;

/// Message parameters to receive via token function call.
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
#[serde(untagged)]
enum TokenReceiverMessage {
    List { allowance: Balance },
    // Change { allowance: Balance }, TODO
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
            // match self.ft_faucet.insert(&env::predecessor_account_id()) {
            //     // token exist update -> the ballance
            //     Some(ft_contract) => {}
            //     // token does not exist -> return to sender
            //     none => PromiseOrValue::Value(amount),
            // }
            log!("Message is empty");

            return PromiseOrValue::Value(amount);
        }
        // if the message is List add it to the ft_faucet HashMap
        let message = serde_json::from_str::<TokenReceiverMessage>(&msg).expect("WRONG MSG FORMAT");

        log!("=================== {:?} =============", &msg);

        match message {
            TokenReceiverMessage::List { allowance } => {
                let promise = ft_contract::ext(env::predecessor_account_id())
                    .with_static_gas(Gas(50 * TGAS))
                    .ft_metadata()
                    .then(
                        Self::ext(env::current_account_id())
                            .with_static_gas(Gas(50 * TGAS))
                            .ft_add_token(env::predecessor_account_id(), amount.0, allowance),
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
        ft_allowance: Balance,
        ft_available: Balance,
    ) -> PromiseOrValue<U128> {
        match call_result {
            Ok(ft_metadata) => {
                // Result is Ok store into ft_faucet HashMap
                self.ft_faucet.insert(
                    ft_account_id,
                    (ft_allowance, ft_available, ft_metadata.clone()),
                );
                // Log successful message
                log!("Successfully added {} with max request allowance of {} and initial amount of {} ", ft_metadata.name, ft_allowance, ft_available);
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
