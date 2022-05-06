// cSpell:ignore yocto, borsh
#[cfg(test)]
use crate::Contract;
use near_sdk::test_utils::{accounts, VMContextBuilder};
use near_sdk::{testing_env, ONE_YOCTO};

fn get_context(is_view: bool) -> VMContextBuilder {
    let mut builder = VMContextBuilder::new();
    builder.account_balance(200).is_view(is_view);
    builder
}

#[test]
fn test_contribute() {
    let mut context = get_context(false);

    let mut contract = Contract {
        top_contributors: vec![(accounts(0), 10), (accounts(1), 5)],
        ..Contract::default()
    };
    // alice is top contributor
    assert_eq!(
        (accounts(0), "10".to_string()),
        contract.get_top_contributors()[0]
    );

    // bobs context
    testing_env!(context
        .signer_account_id(accounts(1))
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
        .signer_account_id(accounts(2))
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
