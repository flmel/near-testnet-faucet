use core::panic;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId, Promise};

// 50Ⓝ in yoctoNEAR
const MAX_WITHRAW_AMOUNT: u128 = 50_000_000_000_000_000_000_000_000;
// hypothetically 27690 * ~1.3s per block = ~10min
const BLOCK_GAP_LIMITER: u64 = 27690;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    top_donators: Vec<(AccountId, u128)>,
    recent_receivers: UnorderedMap<AccountId, u64>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            top_donators: Vec::new(),
            recent_receivers: UnorderedMap::new(b"r".to_vec()),
        }
    }
}

#[near_bindgen]
impl Contract {
    // get ballance available for withdraw
    pub fn get_balance(&self) -> u128 {
        env::account_balance()
    }
    // get top donors
    pub fn get_top_donors(&self) -> Vec<(AccountId, u128)> {
        self.top_donators.clone()
    }

    pub fn request_funds(&mut self, receiver: AccountId, amount: u128) {
        // convert to yoctoNear
        let amount = amount * 10u128.pow(24);
        let current_block_height = env::block_height();

        assert!(
            amount <= MAX_WITHRAW_AMOUNT,
            "Amount is too big, don't be greedy"
        );
        // did the receiver get money recently ? if not insert them in the the map
        match self.recent_receivers.get(&receiver) {
            Some(previous_block_height) => {
                // if they did receive within the last ~10 min block them
                if &current_block_height - &previous_block_height < BLOCK_GAP_LIMITER {
                    panic!("you have to wait for a little longer")
                }
                // else update the block that they made the call
                self.update_or_insert_recent_receivers(&receiver, &current_block_height);
                env::log_str("recent_receivers: Updated");
            }
            None => {
                self.update_or_insert_recent_receivers(&receiver, &current_block_height);
                env::log_str("recent_receivers: Insert New");
            }
        }

        Promise::new(receiver).transfer(amount);
    }

    // donate to the faucet contract to get in the list of fame
    #[payable]
    pub fn donate(&mut self) {
        let donator = env::signer_account_id();
        let amount = env::attached_deposit();

        match self
            .top_donators
            .binary_search_by(|(account_id, _)| account_id.cmp(&donator))
        {
            Ok(index) => self.top_donators[index].1 += amount,
            Err(index) => self.top_donators.insert(index, (donator, amount)),
        }

        self.top_donators.sort_by(|a, b| b.1.cmp(&a.1));
        self.top_donators.truncate(10);

        env::state_write(self); // <- what/why?
    }

    #[private]
    fn update_or_insert_recent_receivers(
        &mut self,
        receiver: &AccountId,
        current_block_height: &u64,
    ) {
        self.recent_receivers
            .insert(&receiver, &current_block_height);
    }
}

// TESTS
#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    fn get_context(is_view: bool) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.account_balance(10).is_view(is_view);
        builder
    }

    #[test]
    fn test_get_balance() {
        let mut context = get_context(true);
        testing_env!(context.signer_account_id(accounts(0)).build());
        let contract = Contract::default();

        assert_eq!(10, contract.get_balance());
    }

    #[test]
    fn test_get_top_donors() {
        let context = get_context(true);
        testing_env!(context.build());
        let contract = Contract::default();
        // initial donors is empty
        assert!(contract.get_top_donors().is_empty());
    }

    #[test]
    fn test_donate() {
        let mut context = get_context(false);

        let mut contract = Contract {
            top_donators: vec![(accounts(0), 10), (accounts(1), 5)],
            ..Contract::default()
        };
        // alice is top contributor
        assert_eq!((accounts(0), 10), contract.get_top_donors()[0]);

        // bobs context
        testing_env!(context
            .signer_account_id(accounts(1))
            .attached_deposit(10)
            .build());
        contract.donate();
        // bob donates 10 (with inital donation of 5
        // he updates his previous donation score and is now top donator)
        assert_eq!((accounts(1), 15), contract.get_top_donors()[0]);

        // charlies context
        testing_env!(context
            .signer_account_id(accounts(2))
            .attached_deposit(11)
            .build());
        contract.donate();
        // charlies donates 11 and goes on second position
        assert_eq!((accounts(2), 11), contract.get_top_donors()[1]);

        // the vec shall be sorted as vec[(bob, 15), (charlie: 11), (alice, 10)]
        assert_eq!(
            vec![(accounts(1), 15), (accounts(2), 11), (accounts(0), 10)],
            contract.get_top_donors()
        )
    }
}
