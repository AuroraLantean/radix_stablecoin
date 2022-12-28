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
pub fn user_balance(
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
    println!("user_balance receipt:{:?}\n", receipt);
    receipt.expect_commit_success();
    receipt.output(1)
}

#[allow(unused)]
pub fn get_data(
    test_runner: &mut TestRunner<TypedInMemorySubstateStore>,
    account: &Account,
    component: ComponentAddress,
) -> (u16, Decimal, Decimal) {
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_method(component, "get_data", args!())
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleAddress::from_public_key(&account.public_key)],
    );
    println!("get_data receipt:{:?}\n", receipt);
    receipt.expect_commit_success();
    println!("get_data receipt success");
    let data: (u16, Decimal, Decimal) = receipt.output(1);
    println!("data:{:?}\n", data);
    data
}

#[allow(unused)]
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

#[allow(unused)]
pub fn invoke_badge_access(
    test_runner: &mut TestRunner<TypedInMemorySubstateStore>,
    account: &Account,
    component: ComponentAddress,
    func_name: &str,
    amount: Decimal,
    badge_addr: ResourceAddress,
    badge_amount: Decimal,
) {
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .create_proof_from_account_by_amount(account.account_component, badge_amount, badge_addr)
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

#[allow(unused)]
pub fn invoke_badge_access_with_bucket(
    test_runner: &mut TestRunner<TypedInMemorySubstateStore>,
    account: &Account,
    component: ComponentAddress,
    func_name: &str,
    amount: Decimal,
    token_addr: ResourceAddress,
    badge_addr: ResourceAddress,
    badge_amount: Decimal,
) {
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .create_proof_from_account_by_amount(account.account_component, badge_amount, badge_addr)
        .withdraw_from_account_by_amount(account.account_component, amount, token_addr)
        .take_from_worktop(token_addr, |builder, bucket_id| {
            builder.call_method(component, func_name, args!(amount, Bucket(bucket_id)))
        })
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

#[allow(unused)]
pub fn burn_in_vault(
    test_runner: &mut TestRunner<TypedInMemorySubstateStore>,
    account: &Account,
    component: ComponentAddress,
    func_name: &str,
    amount: Decimal,
    bucket: Bucket,
    badge_addr: ResourceAddress,
    badge_amount: Decimal,
) {
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .create_proof_from_account_by_amount(account.account_component, badge_amount, badge_addr)
        .call_method(component, func_name, args!(amount, bucket))
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
