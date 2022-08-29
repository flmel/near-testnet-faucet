use crate::external::*;
use crate::*;
use near_contract_standards::fungible_token::{
    metadata::FungibleTokenMetadata, receiver::FungibleTokenReceiver,
};
use near_sdk::{log, Gas, PromiseError, PromiseOrValue};

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        let promise = ft_contract::ext(env::predecessor_account_id())
            .with_static_gas(Gas(50 * TGAS))
            .ft_metadata()
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(Gas(50 * TGAS))
                    .ft_list_token(env::predecessor_account_id()),
                // ^remove arg
            );

        PromiseOrValue::from(promise)
    }
}

#[near_bindgen]
impl Contract {
    #[private]
    pub fn ft_list_token(
        &mut self,
        #[callback_result] call_result: Result<FungibleTokenMetadata, PromiseError>,
        ft_account_id: AccountId,
    ) -> PromiseOrValue<U128> {
        // Check if the promise succeeded by calling the method outlined in external.rs
        // if call_result.is_err() {
        //     log!("There was an error contacting Hello NEAR");
        // }
        match call_result {
            Ok(metadata) => log!("{}", metadata.name),
            Err(err) => log!("{:#?}", err),
        }

        // log!("{}", call_result.unwrap());
        log!("there is no error so far, {}", ft_account_id);
        // self.ft_faucet
        //     .insert(&env::current_account_id(), &(0, call_result));

        PromiseOrValue::Value(U128(0))
    }
}
