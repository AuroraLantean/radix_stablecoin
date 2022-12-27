use radix_engine::ledger::*;
use radix_engine_interface::core::NetworkDefinition;
use radix_engine_interface::model::FromPublicKey;
use scrypto::prelude::*;
use scrypto_unit::*;
use transaction::builder::ManifestBuilder;
use transaction::signing::EcdsaSecp256k1PrivateKey;
pub struct Account {
    pub public_key: EcdsaSecp256k1PublicKey,
    pub private_key: EcdsaSecp256k1PrivateKey,
    pub account_component: ComponentAddress,
}
#[allow(unused)]
pub fn get_balance(
    test_runner: &mut TestRunner<TypedInMemorySubstateStore>,
    account: &Account,
    resource_addr: ResourceAddress,
) -> Decimal {
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_method(account.account_component, "balance", args!(resource_addr))
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleAddress::from_public_key(&account.public_key)],
    );
    println!("get_balance receipt:{:?}\n", receipt);
    receipt.expect_commit_success();
    receipt.output(1)
}

pub fn get_data(
    test_runner: &mut TestRunner<TypedInMemorySubstateStore>,
    account: &Account,
    component: ComponentAddress,
) {
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_method(component, "get_data", args!())
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleAddress::from_public_key(&account.public_key)],
    );
    println!("get_data receipt:{:?}\n", receipt);
    receipt.expect_commit_success();
    let data: (u16, Decimal) = receipt.output(1);
    println!("data:{:?}\n", data);
}

//invoke
pub fn invoke(
    test_runner: &mut TestRunner<TypedInMemorySubstateStore>,
    account: &Account,
    component: ComponentAddress,
    func_name: &str,
    amount: Decimal,
) {
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_method(component, func_name, args!(amount))
        .call_method(
            account.account_component,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleAddress::from_public_key(&account.public_key)],
    );
    println!("{} receipt: {:?}\n", func_name, receipt);
    receipt.expect_commit_success();
    println!("{} ends successfully", func_name);
}

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

    let total_supply = dec!(1000000);
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

    let admin_badge_balance = get_balance(&mut test_runner, &account, resources[0]);
    let wd_badge_balance = get_balance(&mut test_runner, &account, resources[1]);
    let auth_badge_balance = get_balance(&mut test_runner, &account, resources[2]);
    let token_balance = get_balance(&mut test_runner, &account, resources[3]);
    println!(
        "admin_badge_balance:{}, wd_badge_balance:{}, auth_badge_balance:{}, token_balance:{}",
        admin_badge_balance, wd_badge_balance, auth_badge_balance, token_balance
    );
    assert_eq!(admin_badge_balance, dec!(3));
    assert_eq!(wd_badge_balance, dec!(2));
    assert_eq!(auth_badge_balance, dec!(0));
    assert_eq!(token_balance, dec!(0));

    //----------------== get_data
    get_data(&mut test_runner, &account, component);
    //println!("get_data receipt: {:?}\n", receipt);
    println!("get_data receipt success");

    //----------------== mint_to_bucket
    let amount = dec!(1000);
    invoke(
        &mut test_runner,
        &account,
        component,
        "mint_to_bucket",
        amount,
    );

    //----------------== mint_in_vault
    let amount = dec!(1000);
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_method(component, "mint_in_vault", args!(amount))
        // .take_from_worktop(admin_badge_addr???, |builder, bucket_id| {
        //     builder.call_method(component, "mint_in_vault", args!(amount, Bucket(bucket_id)))
        // })
        .call_method(
            account_component,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleAddress::from_public_key(&public_key)],
    );
    println!("mint_in_vault receipt: {:?}\n", receipt);
    receipt.expect_commit_success();
    println!("mint_in_vault receipt success");

    //----------------== get_data
    get_data(&mut test_runner, &account, component);
    //println!("get_data receipt: {:?}\n", receipt);
    println!("get_data receipt success");
}
