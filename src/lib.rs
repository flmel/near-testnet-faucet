use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::LookupSet,
    env,
    json_types::U128,
    near_bindgen, require, AccountId, Promise, ONE_NEAR,
};

use std::collections::HashMap;

// 100Ⓝ in yoctoNEAR
const MAX_WITHDRAW_AMOUNT: u128 = 100 * ONE_NEAR;
// 30min in ms
const REQUEST_GAP_LIMITER: u64 = 1800000;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    top_contributors: Vec<(AccountId, u128)>,
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
    pub fn request_funds(&mut self, receiver_id: AccountId, amount: U128) -> Promise {
        // check if predecessor is in the blacklist
        require!(
            self.blacklist.contains(&env::predecessor_account_id()) == false,
            "Account has been blacklisted!"
        );
        require!(
            amount.0 <= MAX_WITHDRAW_AMOUNT,
            "Withdraw request too large!"
        );
        // TODO: Remove hardcoded bad actor
        if receiver_id.to_string().contains("fengye") {
            env::panic_str("Account has been blacklisted!");
        }
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
        Promise::new(receiver_id.clone()).transfer(amount.0)
    }

    #[private]
    pub fn add_to_blacklist(&mut self, account_id: AccountId) {
        self.blacklist.insert(&account_id);
    }

    #[private]
    pub fn remove_from_blacklist(&mut self, account_id: AccountId) {
        self.blacklist.remove(&account_id);
    }

    // contribute to the faucet contract to get in the list of fame
    #[payable]
    pub fn contribute(&mut self) {
        let donator = env::signer_account_id();
        let amount = env::attached_deposit();

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
}
#[cfg(test)]
mod tests;
