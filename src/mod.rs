use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;
use near_sdk::{ext_contract, json_types::U128, AccountId, PromiseOrValue};

pub const TGAS: u64 = 1_000_000_000_000;

// Interface of this contract, for callbacks
#[ext_contract(this_contract)]
trait Callbacks {
    fn ft_list_token(&mut self, ft_account_id: AccountId) -> PromiseOrValue<U128>;
}

// Interface, for cross-contract calls
#[ext_contract(ft_contract)]
trait FtContract {
    fn ft_metadata(&self) -> FungibleTokenMetadata;
}
