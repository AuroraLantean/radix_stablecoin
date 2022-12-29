use radix_engine::ledger::*;
use radix_engine_interface::core::NetworkDefinition;
use radix_engine_interface::model::FromPublicKey;
use scrypto::prelude::*;
use scrypto_unit::*;
use transaction::builder::ManifestBuilder;
use transaction::signing::EcdsaSecp256k1PrivateKey;

pub struct User {
    pub public_key: EcdsaSecp256k1PublicKey,
    pub private_key: EcdsaSecp256k1PrivateKey,
    pub compo_addr: ComponentAddress,
}

#[allow(unused)]
pub fn make_users(test_runner: &mut TestRunner<TypedInMemorySubstateStore>) -> (User, User, User) {
    let user1 = make_one_user(test_runner);
    let user2 = make_one_user(test_runner);
    let user3 = make_one_user(test_runner);
    (user1, user2, user3)
}
#[allow(unused)]
pub fn make_one_user(test_runner: &mut TestRunner<TypedInMemorySubstateStore>) -> User {
    let (public_key, private_key, compo_addr) = test_runner.new_allocated_account();
    User {
        public_key,
        private_key,
        compo_addr,
    }
}

#[allow(unused)]
pub fn deploy_blueprint(
    test_runner: &mut TestRunner<TypedInMemorySubstateStore>,
    blueprint_name: &str,
    init_func_name: &str,
    decimal1: Decimal,
    user: &User,
) -> (Vec<ResourceAddress>, ComponentAddress) {
    println!("--------== deploy_blueprint");
    let package_address = test_runner.compile_and_publish(this_package!());
    //publish_package_with_owner
    println!("check1");
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_function(
            package_address,
            blueprint_name,
            init_func_name,
            args!(decimal1),
        )
        .call_method(
            user.compo_addr,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
        .build();
    println!("check2");
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleAddress::from_public_key(&user.public_key)],
    );
    println!("{}() receipt: {:?}\n", init_func_name, receipt);

    receipt.expect_commit_success();
    let component = receipt
        .expect_commit()
        .entity_changes
        .new_component_addresses[0];
    println!("{}() receipt success", init_func_name);
    let resources = receipt.new_resource_addresses();
    ((*resources).clone(), component)
}

#[allow(unused)]
pub fn user_balance(
    test_runner: &mut TestRunner<TypedInMemorySubstateStore>,
    user: &User,
    resource_addr: ResourceAddress,
) -> Decimal {
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_method(user.compo_addr, "balance", args!(resource_addr))
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleAddress::from_public_key(&user.public_key)],
    );
    println!("user_balance receipt:{:?}\n", receipt);
    receipt.expect_commit_success();
    receipt.output(1)
}

#[allow(unused)]
pub fn get_data(
    test_runner: &mut TestRunner<TypedInMemorySubstateStore>,
    user: &User,
    component: ComponentAddress,
) -> (u16, Decimal, Decimal) {
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_method(component, "get_data", args!())
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleAddress::from_public_key(&user.public_key)],
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
    user: &User,
    component: ComponentAddress,
    func_name: &str,
    amount: Decimal,
) {
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_method(component, func_name, args!(amount))
        .call_method(
            user.compo_addr,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleAddress::from_public_key(&user.public_key)],
    );
    println!("{} receipt: {:?}\n", func_name, receipt);
    receipt.expect_commit_success();
    println!("{} ends successfully", func_name);
}

#[allow(unused)]
pub fn invoke_badge_access(
    test_runner: &mut TestRunner<TypedInMemorySubstateStore>,
    user: &User,
    component: ComponentAddress,
    func_name: &str,
    amount: Decimal,
    badge_addr: ResourceAddress,
    badge_amount: Decimal,
) {
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .create_proof_from_account_by_amount(user.compo_addr, badge_amount, badge_addr)
        .call_method(component, func_name, args!(amount))
        .call_method(
            user.compo_addr,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleAddress::from_public_key(&user.public_key)],
    );
    println!("{} receipt: {:?}\n", func_name, receipt);
    receipt.expect_commit_success();
    println!("{} ends successfully", func_name);
}

#[allow(unused)]
pub fn invoke_badge_access_with_bucket(
    test_runner: &mut TestRunner<TypedInMemorySubstateStore>,
    user: &User,
    component: ComponentAddress,
    func_name: &str,
    amount: Decimal,
    token_addr: ResourceAddress,
    badge_addr: ResourceAddress,
    badge_amount: Decimal,
) {
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .create_proof_from_account_by_amount(user.compo_addr, badge_amount, badge_addr)
        .withdraw_from_account_by_amount(user.compo_addr, amount, token_addr)
        .take_from_worktop_by_amount(amount, token_addr, |builder, bucket_id| {
            builder.call_method(component, func_name, args!(Bucket(bucket_id)))
        })
        .call_method(
            user.compo_addr,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleAddress::from_public_key(&user.public_key)],
    );
    println!("{} receipt: {:?}\n", func_name, receipt);
    receipt.expect_commit_success();
    println!("{} ends successfully", func_name);
}

#[allow(unused)]
pub fn burn_in_bucket(
    test_runner: &mut TestRunner<TypedInMemorySubstateStore>,
    user: &User,
    component: ComponentAddress,
    func_name: &str,
    amount: Decimal,
    token_addr: ResourceAddress,
    badge_addr: ResourceAddress,
    badge_amount: Decimal,
) {
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .create_proof_from_account_by_amount(user.compo_addr, badge_amount, badge_addr)
        .withdraw_from_account_by_amount(user.compo_addr, amount, token_addr)
        .take_from_worktop_by_amount(amount, token_addr, |builder, bucket_id| {
            builder.call_method(component, func_name, args!(Bucket(bucket_id)))
        })
        .call_method(
            user.compo_addr,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleAddress::from_public_key(&user.public_key)],
    );
    println!("{} receipt: {:?}\n", func_name, receipt);
    receipt.expect_commit_success();
    println!("{} ends successfully", func_name);
}

#[allow(unused)]
pub fn burn_in_vault(
    test_runner: &mut TestRunner<TypedInMemorySubstateStore>,
    user: &User,
    component: ComponentAddress,
    func_name: &str,
    amount: Decimal,
    bucket: Bucket,
    badge_addr: ResourceAddress,
    badge_amount: Decimal,
) {
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .create_proof_from_account_by_amount(user.compo_addr, badge_amount, badge_addr)
        .call_method(component, func_name, args!(amount, bucket))
        .call_method(
            user.compo_addr,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleAddress::from_public_key(&user.public_key)],
    );
    println!("{} receipt: {:?}\n", func_name, receipt);
    receipt.expect_commit_success();
    println!("{} ends successfully", func_name);
}

#[allow(unused)]
pub fn send_tokens(
    test_runner: &mut TestRunner<TypedInMemorySubstateStore>,
    user_from: &User,
    user_to: &User,
    component: ComponentAddress,
    amount: Decimal,
    token_addr: ResourceAddress,
) {
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .withdraw_from_account_by_amount(user_from.compo_addr, amount, token_addr)
        .take_from_worktop_by_amount(amount, token_addr, |builder, bucket_id| {
            builder.call_method(user_to.compo_addr, "deposit", args!(Bucket(bucket_id)))
        })
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleAddress::from_public_key(&user_from.public_key)],
    );
    println!("send_token receipt: {:?}\n", receipt);
    receipt.expect_commit_success();
    println!("send_token ends successfully");
}
