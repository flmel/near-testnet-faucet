use near_sdk::{
    assert_self,
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::LookupSet,
    env, ext_contract,
    json_types::U128,
    near_bindgen, require, AccountId, Balance, Promise, ONE_NEAR,
};

use std::collections::HashMap;

// settings
const MAX_WITHDRAW_AMOUNT: Balance = 10 * ONE_NEAR;
const REQUEST_GAP_LIMITER: u64 = 3600000;
const VAULT_ID: &str = "vault.nonofficial.testnet";
const MIN_BALANCE_THRESHOLD: Balance = 5000 * ONE_NEAR;

#[ext_contract(vault_contract)]
trait VaultContract {
    fn request_funds(&mut self);
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    recent_contributions: Vec<(AccountId, Balance)>,
    recent_receivers: HashMap<AccountId, u64>,
    // <contract.address.near (TokenName, AmountAvaiable/transfered/, maxAmountAllowed)
    ft_faucet: HashMap<AccountId, (String, U128, U128)>,
    blacklist: LookupSet<AccountId>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            recent_contributions: Vec::new(),
            recent_receivers: HashMap::new(),
            ft_faucet: HashMap::new(),
            blacklist: LookupSet::new(b"s"),
        }
    }
}

#[near_bindgen]
impl Contract {
    // TODO maybe make use of the same function dispense both near and FTs
    pub fn request_funds(&mut self, receiver_id: AccountId, amount: U128) {
        // check if predecessor is in the blacklist
        require!(
            self.blacklist.contains(&env::predecessor_account_id()) == false,
            "Account has been blacklisted!"
        );
        require!(
            amount.0 <= MAX_WITHDRAW_AMOUNT,
            "Withdraw request too large!"
        );

        let current_timestamp_ms: u64 = env::block_timestamp_ms();

        // purge expired restrictions
        self.recent_receivers
            .retain(|_, v: &mut u64| *v + REQUEST_GAP_LIMITER > current_timestamp_ms);

        // did the receiver get money recently? if not insert them in the the map
        match self.recent_receivers.get(&receiver_id) {
            Some(previous_timestamp_ms) => {
                // if they did receive within the last ~30 min block them
                if &current_timestamp_ms - previous_timestamp_ms < REQUEST_GAP_LIMITER {
                    env::panic_str(
                        "You have to wait for a little longer before requesting to this account!",
                    )
                }
            }
            None => {
                self.recent_receivers
                    .insert(receiver_id.clone(), current_timestamp_ms);
            }
        }
        // make the transfer
        Promise::new(receiver_id.clone()).transfer(amount.0);
        // check if additional liquidity is needed
        if env::account_balance() < MIN_BALANCE_THRESHOLD {
            self.request_additional_liquidity();
        }
    }
    // TODO
    // Use this to populate the ft_faucet hashmap, the msg can be used for the max amount that the caller wants go giveaway per request
    fn ft_on_transfer(&mut self, sender_id: AccountId, amount: U128, msg: String) -> U128 {
        U128(0)
    }
    // TODO
    // Use this to change the entry in the map, request the ft contract to be the sender to amend the given entry from the hashmap
    pub fn ft_change_allowance(&mut self) {}
    // TODO
    // USE this to make XCC call to the request contract to transfer FT to the user
    // the faucet should cover storage cost for user on ft contract ???
    pub fn ft_request_funds(&mut self) {}

    // #[private] this macro does not expand for unit testing therefore I'm ignoring it for the time being
    pub fn add_to_blacklist(&mut self, account_id: AccountId) {
        assert_self();
        self.blacklist.insert(&account_id);
    }

    pub fn batch_add_to_blacklist(&mut self, accounts: Vec<AccountId>) {
        assert_self();
        // sadly no append TODO: Optimise
        for account in accounts {
            self.blacklist.insert(&account);
        }
    }

    // #[private] this macro does not expand for unit testing therefore I'm ignoring it for the time being
    pub fn remove_from_blacklist(&mut self, account_id: AccountId) {
        assert_self();
        self.blacklist.remove(&account_id);
    }

    // #[private] this macro does not expand for unit testing therefore I'm ignoring it for the time being
    pub fn clear_recent_receivers(&mut self) {
        assert_self();
        self.recent_receivers.clear();
    }

    // contribute to the faucet contract to get in the list of fame
    #[payable]
    pub fn contribute(&mut self) {
        let contributor: AccountId = env::predecessor_account_id();
        let amount: Balance = env::attached_deposit();

        self.recent_contributions.insert(0, (contributor, amount));
        self.recent_contributions.truncate(10);
    }

    // get top contributors
    pub fn get_recent_contributions(&self) -> Vec<(AccountId, String)> {
        self.recent_contributions
            .iter()
            .map(|(account_id, amount)| (account_id.clone(), amount.to_string()))
            .collect()
    }

    // request_additional_liquidity
    fn request_additional_liquidity(&self) {
        vault_contract::ext(VAULT_ID.parse().unwrap()).request_funds();
    }
}
#[cfg(test)]
mod tests;
