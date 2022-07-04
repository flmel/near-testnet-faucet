use near_sdk::{
    assert_self,
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::LookupSet,
    env, ext_contract,
    json_types::U128,
    near_bindgen, require, AccountId, Balance, Gas, Promise, ONE_NEAR,
};

use std::collections::HashMap;

const TGAS: u64 = 1_000_000_000_000;
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
    top_contributors: Vec<(AccountId, Balance)>,
    recent_receivers: HashMap<AccountId, u64>,
    blacklist: LookupSet<AccountId>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            top_contributors: Vec::new(),
            recent_receivers: HashMap::new(),
            blacklist: LookupSet::new(b"s"),
        }
    }
}

#[near_bindgen]
impl Contract {
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

    // #[private] this macro does not expand for unit testing therefore I'm ignoring it for the time being
    pub fn add_to_blacklist(&mut self, account_id: AccountId) {
        assert_self();
        self.blacklist.insert(&account_id);
    }

    // #[private] this macro does not expand for unit testing therefore I'm ignoring it for the time being
    pub fn remove_from_blacklist(&mut self, account_id: AccountId) {
        assert_self();
        self.blacklist.remove(&account_id);
    }

    pub fn fix_contribute_vec(&mut self) {
        assert_self();
    }

    // #[private] this macro does not expand for unit testing therefore I'm ignoring it for the time being
    pub fn clear_recent_receivers(&mut self) {
        assert_self();
        self.recent_receivers.clear();
    }

    // contribute to the faucet contract to get in the list of fame
    #[payable]
    pub fn contribute(&mut self) {
        let donator: AccountId = env::predecessor_account_id();
        let amount: Balance = env::attached_deposit();

        match self
            .top_contributors
            .binary_search_by(|(account_id, _)| account_id.cmp(&donator))
        {
            Ok(index) => self.top_contributors[index].1 += amount,
            Err(index) => self.top_contributors.insert(index, (donator, amount)),
        }

        self.top_contributors.sort_by(|a, b| b.1.cmp(&a.1));
        self.top_contributors.truncate(10);

        env::state_write(self);
    }

    // get top contributors
    pub fn get_top_contributors(&self) -> Vec<(AccountId, String)> {
        self.top_contributors
            .iter()
            .map(|(account_id, amount)| (account_id.clone(), amount.to_string()))
            .collect()
    }

    // request_additional_liquidity
    fn request_additional_liquidity(&self) {
        vault_contract::request_funds(VAULT_ID.parse().unwrap(), 0, Gas(5 * TGAS));
    }
}
#[cfg(test)]
mod tests;
