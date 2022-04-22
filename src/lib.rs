use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, near_bindgen, require, AccountId, Promise};
use std::collections::HashMap;

// 50Ⓝ in yoctoNEAR
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
    pub fn get_top_contributors(&self) -> Vec<(AccountId, u128)> {
        self.top_contributors.clone()
    }

    pub fn request_funds(&mut self, receiver: AccountId, amount: u128) -> Promise {
        // convert to yoctoNear
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

        env::state_write(self); // <- what/why?
    }
}

// // TESTS
// #[cfg(all(test, not(target_arch = "wasm32")))]
// mod tests {
//     use super::*;
//     use near_sdk::test_utils::{accounts, VMContextBuilder};
//     use near_sdk::testing_env;

//     fn get_context(is_view: bool) -> VMContextBuilder {
//         let mut builder = VMContextBuilder::new();
//         builder.account_balance(10).is_view(is_view);
//         builder
//     }

//     #[test]
//     fn test_get_balance() {
//         let mut context = get_context(true);
//         testing_env!(context.signer_account_id(accounts(0)).build());
//         let contract = Contract::default();

//         assert_eq!(10, contract.get_balance());
//     }

//     #[test]
//     fn test_get_top_donors() {
//         let context = get_context(true);
//         testing_env!(context.build());
//         let contract = Contract::default();
//         // initial donors is empty
//         assert!(contract.get_top_donors().is_empty());
//     }

//     #[test]
//     fn test_donate() {
//         let mut context = get_context(false);

//         let mut contract = Contract {
//             top_donators: vec![(accounts(0), 10), (accounts(1), 5)],
//             ..Contract::default()
//         };
//         // alice is top contributor
//         assert_eq!((accounts(0), 10), contract.get_top_donors()[0]);

//         // bobs context
//         testing_env!(context
//             .signer_account_id(accounts(1))
//             .attached_deposit(10)
//             .build());
//         contract.donate();
//         // bob donates 10 (with inital donation of 5
//         // he updates his previous donation score and is now top donator)
//         assert_eq!((accounts(1), 15), contract.get_top_donors()[0]);

//         // charlies context
//         testing_env!(context
//             .signer_account_id(accounts(2))
//             .attached_deposit(11)
//             .build());
//         contract.donate();
//         // charlies donates 11 and goes on second position
//         assert_eq!((accounts(2), 11), contract.get_top_donors()[1]);

//         // the vec shall be sorted as vec[(bob, 15), (charlie: 11), (alice, 10)]
//         assert_eq!(
//             vec![(accounts(1), 15), (accounts(2), 11), (accounts(0), 10)],
//             contract.get_top_donors()
//         )
//     }
// }
