use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, near_bindgen, require, AccountId, Promise};
use std::collections::HashMap;

// 100Ⓝ in yoctoNEAR
const MAX_WITHRAW_AMOUNT: u128 = 100_000_000_000_000_000_000_000_000;
const BLOCK_GAP_LIMITER: u64 = 300;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    top_contributors: Vec<(AccountId, u128)>,
    recent_receivers: HashMap<AccountId, u64>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            top_contributors: Vec::new(),
            recent_receivers: HashMap::new(),
        }
    }
}

#[near_bindgen]
impl Contract {
    // get top contributors
    pub fn get_top_contributors(&self) -> Vec<(AccountId, String)> {
        self.top_contributors
            .iter()
            .map(|(account_id, amount)| (account_id.clone(), amount.to_string()))
            .collect()
    }

    pub fn request_funds(&mut self, receiver: AccountId, amount: u128) -> Promise {
        // convert amount to yoctoNEAR
        let amount = amount * 10u128.pow(24);
        let current_block_height = env::block_height();
        // purge expired restrictions
        self.recent_receivers
            .retain(|_, v| *v + BLOCK_GAP_LIMITER > current_block_height);

        require!(amount <= MAX_WITHRAW_AMOUNT, "Withraw request too large!");

        // did the receiver get money recently ? if not insert them in the the map
        match self.recent_receivers.get(&receiver) {
            Some(previous_block_height) => {
                // if they did receive within the last ~10 min block them
                if &current_block_height - previous_block_height < BLOCK_GAP_LIMITER {
                    env::panic_str(
                        "You have to wait for a little longer before requesting to this account!",
                    )
                }
            }
            None => {
                self.recent_receivers
                    .insert(receiver.clone(), current_block_height);
                env::log_str("recent_receivers: Insert New");
            }
        }
        log!("Attemping to transfer {} to {}", amount, receiver);

        Promise::new(receiver.clone()).transfer(amount)
    }

    // donate to the faucet contract to get in the list of fame
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
}
