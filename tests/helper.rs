use radix_engine::{transaction::TransactionReceipt, vm::NoExtension};
use radix_engine_stores::memory_db::InMemorySubstateDatabase;
use scrypto::prelude::*;
use scrypto_unit::*;
use transaction::{builder::ManifestBuilder, prelude::Secp256k1PrivateKey};
pub struct User {
    pub public_key: Secp256k1PublicKey,
    pub private_key: Secp256k1PrivateKey,
    pub compo_addr: ComponentAddress,
}

#[allow(unused)]
pub fn make_users(
    test_runner: &mut TestRunner<NoExtension, InMemorySubstateDatabase>,
) -> (User, User, User) {
    println!("---checkpoint-make_users");
    let user1 = make_one_user(test_runner);
    let user2 = make_one_user(test_runner);
    let user3 = make_one_user(test_runner);
    (user1, user2, user3)
}
#[allow(unused)]
pub fn make_one_user(test_runner: &mut TestRunner<NoExtension, InMemorySubstateDatabase>) -> User {
    println!("---checkpoint-make_one_user");
    let (public_key, private_key, compo_addr) = test_runner.new_allocated_account();
    User {
        public_key,
        private_key,
        compo_addr,
    }
}
#[allow(unused)]
pub fn make_badge(badge_name: &str) -> FungibleBucket {
    let owner_badge: FungibleBucket = ResourceBuilder::new_fungible(OwnerRole::None)
        .divisibility(DIVISIBILITY_NONE)
        .metadata(metadata!(
            init {
                "name" => badge_name, locked;
            }
        ))
        .mint_initial_supply(1);

    owner_badge
}

pub fn get_nft_gid(pk: &Secp256k1PublicKey) -> NonFungibleGlobalId {
    NonFungibleGlobalId::from_public_key(pk)
}
#[allow(unused)]
pub fn instantiate_blueprint(
    test_runner: &mut TestRunner<NoExtension, InMemorySubstateDatabase>,
    package_addr: PackageAddress,
    blueprint_name: &str,
    func_name: &str,
    total_supply: Decimal,
    keys_owned: Vec<String>,
    values_owned: Vec<String>,
    user: &User,
    owner_badge_addr: ResourceAddress,
) -> (IndexSet<ResourceAddress>, ComponentAddress) {
    println!("--------== instantiate_blueprint");

    //publish_package_with_owner
    println!("check1");
    let manifest = ManifestBuilder::new()
        .call_function(
            package_addr,
            blueprint_name,
            func_name,
            manifest_args!(total_supply, keys_owned, values_owned, owner_badge_addr),
        )
        .call_method(
            user.compo_addr,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop),
        )
        .build();
    println!("check2");
    let receipt =
        test_runner.execute_manifest_ignoring_fee(manifest, vec![get_nft_gid(&user.public_key)]);

    receipt.expect_commit_success();
    println!("{} receipt success", func_name);

    let compo_addr = receipt.expect_commit(true).new_component_addresses()[0];

    //println!("{}() receipt: {:?}\n", func_name, receipt);
    let resources = receipt.expect_commit(true).new_resource_addresses();
    println!("{} ends successfully", func_name);

    /*mismatched types
    expected struct `std::vec::Vec<scrypto::prelude::ResourceAddress>`
       found struct `IndexSet<scrypto::prelude::ResourceAddress> */
    ((*resources).clone(), compo_addr)
}

#[allow(unused)]
pub fn get_user_balc(
    test_runner: &mut TestRunner<NoExtension, InMemorySubstateDatabase>,
    user: &User,
    resource_addr: ResourceAddress,
    resource_name: &str,
) -> Decimal {
    let balance = test_runner.get_component_balance(user.compo_addr, resource_addr);
    println!("{} balance:{:?}", resource_name, balance);

    balance
}
#[allow(unused)]
pub fn get_vault_balc(
    test_runner: &mut TestRunner<NoExtension, InMemorySubstateDatabase>,
    compo_addr: ComponentAddress,
    resource_addr: ResourceAddress,
    resource_name: &str,
) -> Vec<Decimal> {
    println!("get_vault_balc for {} @ {:?}", resource_name, resource_addr);
    let vault_ids = test_runner.get_component_vaults(compo_addr, resource_addr);
    println!("vault_ids: {:?}", vault_ids);

    let balcs: Vec<Decimal> = vault_ids
        .iter()
        .map(|vault_id| {
            let balc: Option<Decimal> = test_runner.inspect_vault_balance(*vault_id);
            println!("balance of vault_id {:?}:{:?}", vault_id, balc);
            balc.map_or(dec!(0), |v| v)
        })
        .collect();
    balcs
}

#[allow(unused)]
pub fn get_vault_data(
    test_runner: &mut TestRunner<NoExtension, InMemorySubstateDatabase>,
    user: &User,
    compo_addr: ComponentAddress,
) -> (u16, Decimal, Decimal) {
    println!("--------== get_vault_data");
    let manifest = ManifestBuilder::new()
        .call_method(compo_addr, "get_vault_data", manifest_args!())
        .build();
    println!("check1");
    let receipt = test_runner.execute_manifest_ignoring_fee(manifest, vec![]); //get_nft_gid(&user.public_key)
    println!("check2");
    //receipt.expect_commit_success();

    //println!("get_vault_data() receipt:{:?}\n", receipt);
    let data: (u16, Decimal, Decimal) = receipt.expect_commit(true).output(1);
    //let data: (u16, Decimal, Decimal) = receipt.expect_commit_success().output(1);
    println!("data:{:?}\n", data);
    data
}

#[allow(unused)]
pub fn call_function(
    test_runner: &mut TestRunner<NoExtension, InMemorySubstateDatabase>,
    user: &User,
    package_address: PackageAddress,
    blueprint_name: &str,
    func_name: &str,
    token_addr: ResourceAddress,
    keys_owned: Vec<String>,
) -> TransactionReceipt {
    let manifest = ManifestBuilder::new()
        .call_function(
            package_address,
            blueprint_name,
            func_name,
            manifest_args!(token_addr, keys_owned),
        )
        .build();
    let receipt =
        test_runner.execute_manifest_ignoring_fee(manifest, vec![get_nft_gid(&user.public_key)]);
    //println!("{} receipt:{:?}\n", func_name, receipt);
    receipt.expect_commit_success();
    println!("{} receipt success", func_name);
    receipt
}

#[allow(unused)]
pub fn invoke(
    test_runner: &mut TestRunner<NoExtension, InMemorySubstateDatabase>,
    user: &User,
    compo_addr: ComponentAddress,
    func_name: &str,
    amount: Decimal,
) {
    let manifest = ManifestBuilder::new()
        .call_method(compo_addr, func_name, manifest_args!(amount))
        .call_method(
            user.compo_addr,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop),
        )
        .build();
    let receipt =
        test_runner.execute_manifest_ignoring_fee(manifest, vec![get_nft_gid(&user.public_key)]);
    //println!("{} receipt: {:?}\n", func_name, receipt);
    receipt.expect_commit_success();
    println!("{} ends successfully", func_name);
}

#[allow(unused)]
pub fn invoke_badge_access_decimal(
    test_runner: &mut TestRunner<NoExtension, InMemorySubstateDatabase>,
    user: &User,
    compo_addr: ComponentAddress,
    func_name: &str,
    amount: Decimal,
    badge_addr: ResourceAddress,
    badge_amount: Decimal,
) {
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_amount(user.compo_addr, badge_addr, badge_amount)
        .call_method(compo_addr, func_name, manifest_args!(amount))
        .call_method(
            user.compo_addr,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop),
        )
        .build();
    let receipt =
        test_runner.execute_manifest_ignoring_fee(manifest, vec![get_nft_gid(&user.public_key)]);
    //println!("{} receipt: {:?}\n", func_name, receipt);
    receipt.expect_commit_success();
    println!("{} ends successfully", func_name);
}

#[allow(unused)]
pub fn invoke_badge_access(
    test_runner: &mut TestRunner<NoExtension, InMemorySubstateDatabase>,
    user: &User,
    compo_addr: ComponentAddress,
    func_name: &str,
    badge_addr: ResourceAddress,
    badge_amount: Decimal,
) {
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_amount(user.compo_addr, badge_addr, badge_amount)
        .call_method(compo_addr, func_name, manifest_args!())
        .call_method(
            user.compo_addr,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop),
        )
        .build();
    let receipt =
        test_runner.execute_manifest_ignoring_fee(manifest, vec![get_nft_gid(&user.public_key)]);
    //println!("{} receipt: {:?}\n", func_name, receipt);
    receipt.expect_commit_success();
    println!("{} ends successfully", func_name);
}

#[allow(unused)]
pub fn invoke_badge_access_with_bucket(
    test_runner: &mut TestRunner<NoExtension, InMemorySubstateDatabase>,
    user: &User,
    compo_addr: ComponentAddress,
    func_name: &str,
    amount: Decimal,
    token_addr: ResourceAddress,
    badge_addr: ResourceAddress,
    badge_amount: Decimal,
) {
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_amount(user.compo_addr, badge_addr, badge_amount)
        .withdraw_from_account(user.compo_addr, token_addr, amount)
        .take_from_worktop(token_addr, amount, "bucket")
        .call_method(
            user.compo_addr,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop),
        )
        .build();
    /**|builder, bucket| {
        builder.call_method(compo_addr, func_name, manifest_args!(bucket))
    } */
    let receipt =
        test_runner.execute_manifest_ignoring_fee(manifest, vec![get_nft_gid(&user.public_key)]);
    //println!("{} receipt: {:?}\n", func_name, receipt);
    receipt.expect_commit_success();
    println!("{} ends successfully", func_name);
}

#[allow(unused)]
pub fn update_metadata(
    test_runner: &mut TestRunner<NoExtension, InMemorySubstateDatabase>,
    user: &User,
    compo_addr: ComponentAddress,
    key: String,
    value: String,
    badge_addr: ResourceAddress,
    badge_amount: Decimal,
) {
    let func_name = "update_metadata";
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_amount(user.compo_addr, badge_addr, badge_amount)
        .call_method(compo_addr, func_name, manifest_args!(key, value))
        .call_method(
            user.compo_addr,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop),
        )
        .build();
    let receipt =
        test_runner.execute_manifest_ignoring_fee(manifest, vec![get_nft_gid(&user.public_key)]);
    //println!("{} receipt: {:?}\n", func_name, receipt);
    receipt.expect_commit_success();
    println!("{} ends successfully", func_name);
}

#[allow(unused)]
pub fn convert_str_slices(str_vec: Vec<&str>) -> Vec<String> {
    let mut owned_strings: Vec<String> = vec![];
    for i in str_vec {
        owned_strings.push(i.to_owned());
    }
    owned_strings
}
#[allow(unused)]
pub fn vec_option_string(option_vec: Vec<Option<String>>) -> Vec<String> {
    let mut owned_strings: Vec<String> = vec![];
    for e in option_vec {
        if e.is_some() {
            owned_strings.push(e.unwrap());
        } else {
            owned_strings.push("---".to_owned());
        }
    }
    owned_strings
}

//floats and NaNs won't compare! use a tolerance for comparing the other values.
#[allow(unused)]
pub fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}

#[allow(unused)]
pub fn find_replace<T: PartialEq>(mut v: Vec<T>, key: &T, value: T) -> (Vec<T>, T) {
    let index = v.iter().position(|r| *r == *key).unwrap();
    let replaced = std::mem::replace(&mut v[index], value);
    (v, replaced)
}

#[allow(unused)]
pub fn find_replace_two_vec<T: PartialEq>(
    mut v: Vec<T>,
    keys: &Vec<T>,
    key: &T,
    value: T,
) -> (Vec<T>, T) {
    let index = keys.iter().position(|r| *r == *key).unwrap();
    let replaced = std::mem::replace(&mut v[index], value);
    (v, replaced)
}

#[allow(unused)]
pub fn send_tokens(
    test_runner: &mut TestRunner<NoExtension, InMemorySubstateDatabase>,
    user_from: &User,
    user_to: &User,
    compo_addr: ComponentAddress,
    amount: Decimal,
    token_addr: ResourceAddress,
) {
    let manifest = ManifestBuilder::new()
        .withdraw_from_account(user_from.compo_addr, token_addr, amount)
        .take_all_from_worktop(token_addr, "bucket")
        .build();
    /*|builder, bucket| {
    builder.call_method(user_to.compo_addr, "deposit", manifest_args!(bucket))     */
    let receipt = test_runner
        .execute_manifest_ignoring_fee(manifest, vec![get_nft_gid(&user_from.public_key)]);
    println!("send_token receipt: {:?}\n", receipt);
    receipt.expect_commit_success();
    println!("send_token ends successfully");
}
