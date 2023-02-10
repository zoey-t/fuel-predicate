use fuels::prelude::*;

// Load predicate abi from json
abigen!(Predicate(
    name = "SimplePredicate",
    abi = "out/debug/fuel-predicate-abi.json",
),);

#[tokio::test]
async fn test_simple_predicate() {
    // setup wallet
    setup_contract_test!(Wallets("wallet1", "wallet2"));

    // check wallet balance
    let asset_id = AssetId::default();
    let balance = wallet1.get_asset_balance(&asset_id).await.unwrap();

    println!("wallet1 balance{}:", balance);

    // create a SimplePredicate instance
    let predicate = SimplePredicate::load_from("out/debug/fuel-predicate.bin").unwrap();

    // transferr funds to predicate
    let amount = 1234u64;
    predicate.receive(&wallet1, amount, asset_id, None).await.ok(); // put .ok() to ignore used result (by making it an Option)

    // check predicate balance
    let provider = wallet1.get_provider().unwrap();
    let predicate_balance = provider.get_asset_balance(predicate.address(), asset_id).await.unwrap();
    println!("predicate balance{}:", predicate_balance);
    assert_eq!(predicate_balance, amount);

    // unlock predicate funds and send it to wallet2
    let amount_to_unlock:u64 = 300;
    let balance_before = wallet2.get_asset_balance(&asset_id).await.unwrap();

    predicate.encode_data(111,111).spend(&wallet2, amount_to_unlock, asset_id, None).await.ok();

    let balance_after = wallet2.get_asset_balance(&asset_id).await.unwrap();

    assert_eq!(balance_after-balance_before, amount_to_unlock);

    // check predicate remaining balance
    let predicate_balance = provider.get_asset_balance(predicate.address(), asset_id).await.unwrap();
    println!("predicate new balance{}:", predicate_balance);
    assert_eq!(predicate_balance, amount-amount_to_unlock);

    // should fail if providing wrong data
    let res = predicate.encode_data(1, 2).spend(&wallet2, amount_to_unlock, asset_id, None).await;
    match res {
        Ok(_receipt) => {
            println!("predicate unlocked by providing wrong data!!!")
        },
        Err(err) => {
            println!("expected! predicate should not be unlocked with wrong data\n");
            println!("{:?}", err);
        }
    }
    
}
