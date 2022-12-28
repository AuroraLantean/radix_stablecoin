use radix_engine::ledger::*;
use radix_engine_interface::core::NetworkDefinition;
use radix_engine_interface::model::FromPublicKey;
use scrypto::prelude::*;
use scrypto_unit::*;
use transaction::builder::ManifestBuilder;

mod helper;
pub use helper::*;

#[test]
fn test_stablecoin() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(true, &mut store);

    // Create an account
    let (public_key, private_key, account_component) = test_runner.new_allocated_account();

    let account = Account {
        public_key,
        private_key,
        account_component,
    };

    // Publish packageget_account_balance
    let package_address = test_runner.compile_and_publish(this_package!());

    let total_supply_original = dec!(1000000);
    let mut total_supply = total_supply_original;
    println!("test4");
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_function(package_address, "StableCoin", "new", args!(total_supply))
        .call_method(
            account_component,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
        .build();
    //COMMITTED FAILURE: KernelError(DropNodeFailure(Worktop))
    println!("test5");
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleAddress::from_public_key(&public_key)],
    );
    println!("new() receipt: {:?}\n", receipt);
    //receipt.expect_commit_failure()
    //receipt.expect_specific_failure(f)
    receipt.expect_commit_success();
    let component = receipt
        .expect_commit()
        .entity_changes
        .new_component_addresses[0];
    println!("new() receipt success");
    let resources = receipt.new_resource_addresses();
    println!("new resource addr: {:?}", resources);
    assert_eq!(resources.len(), 4);

    let admin_badge_addr = resources[0];
    let token_addr = resources[3];
    let admin_badge_balance = user_balance(&mut test_runner, &account, admin_badge_addr);
    let wd_badge_balance = user_balance(&mut test_runner, &account, resources[1]);
    let auth_badge_balance = user_balance(&mut test_runner, &account, resources[2]);
    let utoken_bal = user_balance(&mut test_runner, &account, resources[3]);
    println!(
        "admin_badge_balance:{}, wd_badge_balance:{}, auth_badge_balance:{}, utoken_bal:{}",
        admin_badge_balance, wd_badge_balance, auth_badge_balance, utoken_bal
    );
    assert_eq!(admin_badge_balance, dec!(3));
    assert_eq!(wd_badge_balance, dec!(2));
    assert_eq!(auth_badge_balance, dec!(0));
    assert_eq!(utoken_bal, dec!(0));

    //----------------== get_data
    let data = get_data(&mut test_runner, &account, component);
    assert_eq!(data.2, total_supply);

    //----------------== mint_to_bucket
    let amount = dec!(1000);
    let badge_amount = dec!(3);
    invoke_badge_access(
        &mut test_runner,
        &account,
        component,
        "mint_to_bucket",
        amount,
        admin_badge_addr,
        badge_amount,
    );
    let data = get_data(&mut test_runner, &account, component);
    assert_eq!(data.2, total_supply + amount);
    total_supply += amount;
    assert_eq!(data.1, total_supply_original);
    let mut _vault_amount = data.1;
    println!("total supply increased accordingly");

    let utoken_bal = user_balance(&mut test_runner, &account, resources[3]);
    println!("utoken_bal:{}", utoken_bal);
    assert_eq!(utoken_bal, amount);
    let mut _utoken_bal_t = utoken_bal;
    println!("user token balance increased accordingly");

    //----------------== mint_to_vault
    let amount = dec!(1000);
    let badge_amount = dec!(3);
    invoke_badge_access(
        &mut test_runner,
        &account,
        component,
        "mint_to_vault",
        amount,
        admin_badge_addr,
        badge_amount,
    );
    let data = get_data(&mut test_runner, &account, component);
    assert_eq!(data.2, total_supply + amount);
    total_supply += amount;
    assert_eq!(data.1, total_supply_original + amount);
    _vault_amount = total_supply_original + amount;
    println!("vault amount increased accordingly");

    //----------------== withdraw
    let amount = dec!(937);
    let badge_amount = dec!(3);
    invoke_badge_access(
        &mut test_runner,
        &account,
        component,
        "withdraw",
        amount,
        admin_badge_addr,
        badge_amount,
    );
    let data = get_data(&mut test_runner, &account, component);
    assert_eq!(data.2, total_supply);
    assert_eq!(data.1, _vault_amount - amount);
    _vault_amount -= amount;
    println!("vault amount decreased accordingly");

    let utoken_bal = user_balance(&mut test_runner, &account, resources[3]);
    println!("utoken_bal:{}", utoken_bal);
    assert_eq!(utoken_bal, _utoken_bal_t + amount);
    _utoken_bal_t += amount;
    println!("user token balance increased accordingly");

    //----------------== deposit
    /*
        let amount = dec!(37);
        let badge_amount = dec!(3);
        invoke_badge_access_with_bucket(
            &mut test_runner,
            &account,
            component,
            "deposit",
            amount,
            token_addr,
            admin_badge_addr,
            badge_amount,
        );
        let data = get_data(&mut test_runner, &account, component);
        assert_eq!(data.2, total_supply);
        assert_eq!(data.1, _vault_amount + amount);
        _vault_amount += amount;
        println!("vault amount increased accordingly");

        let utoken_bal = user_balance(&mut test_runner, &account, resources[3]);
        println!("utoken_bal:{}", utoken_bal);
        assert_eq!(utoken_bal, _utoken_bal_t - amount);
        _utoken_bal_t -= amount;
        println!("user token balance decreased accordingly");
    */
    //----------------== burn_in_vault
    let amount = dec!(100);
    let badge_amount = dec!(3);
    invoke_badge_access(
        &mut test_runner,
        &account,
        component,
        "burn_in_vault",
        amount,
        admin_badge_addr,
        badge_amount,
    );
    let data = get_data(&mut test_runner, &account, component);
    assert_eq!(data.2, total_supply - amount);
    total_supply -= amount;
    assert_eq!(data.1, _vault_amount - amount);
    _vault_amount -= amount;
    println!("vault amount decreased accordingly");
}
