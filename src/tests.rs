#[cfg(test)]
use crate::Contract;
use near_sdk::test_utils::{accounts, VMContextBuilder};
use near_sdk::{env, testing_env, ONE_NEAR};

fn get_context(is_view: bool) -> VMContextBuilder {
    let mut builder = VMContextBuilder::new();
    builder
        .is_view(is_view)
        .current_account_id("contract.testnet".parse().unwrap());
    builder
}

// contribute
#[test]
fn test_contribute() {
    let mut context = get_context(false);
    let mut contract = Contract::default();

    testing_env!(context
        .account_balance(ONE_NEAR)
        .predecessor_account_id(accounts(0))
        .attached_deposit(ONE_NEAR)
        .build());

    contract.contribute();
    // one near initial + one near contribution
    assert_eq!(env::account_balance(), 2 * ONE_NEAR);
}

// get_top_contributors
#[test]
fn test_get_top_contributors() {
    let mut context = get_context(false);
    let mut contract = Contract {
        top_contributors: vec![(accounts(0), 10), (accounts(1), 5)],
        ..Contract::default()
    };
    // Using small Yocto for simplicity

    // alice is top contributor
    assert_eq!(
        (accounts(0), "10".to_string()),
        contract.get_top_contributors()[0]
    );

    // bobs context
    testing_env!(context
        .predecessor_account_id(accounts(1))
        .attached_deposit(10)
        .build());
    contract.contribute();
    // bob donates 10 (with initial donation of 5
    // he updates his previous donation score and is now top donator)
    assert_eq!(
        (accounts(1), "15".to_string()),
        contract.get_top_contributors()[0]
    );

    // charlies context
    testing_env!(context
        .predecessor_account_id(accounts(2))
        .attached_deposit(11)
        .build());
    contract.contribute();
    // charlies donates 11 and goes on second position
    assert_eq!(
        (accounts(2), "11".to_string()),
        contract.get_top_contributors()[1]
    );

    // the vec shall be sorted as vec[(bob, "15"), (charlie: "11"), (alice, "10")]
    assert_eq!(
        vec![
            (accounts(1), "15".to_string()),
            (accounts(2), "11".to_string()),
            (accounts(0), "10".to_string())
        ],
        contract.get_top_contributors()
    )
}

// add_to_blacklist
#[test]
fn test_add_to_blacklist() {
    let mut context = get_context(false);
    let mut contract = Contract::default();

    testing_env!(context
        .predecessor_account_id("contract.testnet".parse().unwrap())
        .build());

    contract.add_to_blacklist(accounts(1));
    // bob shall be in the blacklist
    assert!(contract.blacklist.contains(&accounts(1)));
}

#[test]
#[should_panic]
fn test_panics_add_to_blacklist() {
    let context = get_context(false);
    let mut contract = Contract::default();

    testing_env!(context.build());
    // self is not the predecessor we shall panic!
    contract.add_to_blacklist(accounts(1));
}

// remove_from_blacklist
#[test]
fn test_remove_from_blacklist() {
    let mut context = get_context(false);
    let mut contract = Contract::default();

    testing_env!(context
        .predecessor_account_id("contract.testnet".parse().unwrap())
        .build());

    contract.remove_from_blacklist(accounts(1));
    // bob shall not be in the the blacklist
    assert!(contract.blacklist.contains(&accounts(0)) == false);
}

#[test]
#[should_panic]
fn test_panics_remove_from_blacklist() {
    let mut context = get_context(false);
    let mut contract = Contract::default();

    testing_env!(context.predecessor_account_id(accounts(0)).build());
    // self is not the predecessor we shall panic!
    contract.remove_from_blacklist(accounts(1));
}
