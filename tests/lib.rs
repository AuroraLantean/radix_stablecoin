use scrypto::prelude::*;
use scrypto_unit::*;
//use transaction::builder::ManifestBuilder;

mod helper;
use helper::*;

#[test]
fn test_stablecoin() {
    // Setup the environment
    let mut test_runner = TestRunnerBuilder::new().build();
    println!("---checkpoint-1");

    let (public_key, private_key, account) = test_runner.new_allocated_account();
    println!("---checkpoint-1b");
    let user1 = User {
        public_key,
        private_key,
        compo_addr: account,
    };
    println!("---checkpoint-1c");
    let (public_key2, private_key2, account2) = test_runner.new_allocated_account();
    let user2 = User {
        public_key: public_key2,
        private_key: private_key2,
        compo_addr: account2,
    };

    // Create an accounts
    //let (user1, user2, user3) = make_users(&mut test_runner);
    println!("---checkpoint-2");

    // Publish package
    let package_address = test_runner.compile_and_publish(this_package!());
    println!("package_address: {:?}", package_address);

    let total_supply_original = dec!(1000000);
    let mut u1token_exp = dec!(0);
    let mut u2token_exp = dec!(0);
    let mut v1token_exp = total_supply_original;

    let keys_str = vec!["name", "symbol", "icon_url", "url", "author", "stage"];
    let keys_owned = convert_str_slices(keys_str);
    let blueprint_name = "StableCoinVault";
    let mut total_supply = total_supply_original;
    let token_name = "USD Tether";
    let token_symbol = "USDT";
    let icon_url = "https://token_website.com/icon.ico";
    let url = "https://token_website.com";
    let author = "Xman";
    let stage = "stage 1 - Fixed supply, withdraw may be restricted";

    let values_str = vec![token_name, token_symbol, icon_url, url, author, stage];
    let values_owned = convert_str_slices(values_str);
    println!("---checkpoint-4");

    let (resources, compo_addr) = instantiate_blueprint(
        &mut test_runner,
        package_address,
        blueprint_name,
        "new",
        total_supply_original,
        keys_owned.clone(),
        values_owned.clone(),
        &user1,
    );
    println!("---checkpoint-5");
    println!("resources: {:?}", resources);
    println!("compo_addr: {:?}", compo_addr);
    assert_eq!(resources.len(), 4);

    let admin_badge_addr = resources[0];
    let wd_badge_addr = resources[1];
    let auth_badge_addr = resources[2];
    let token_addr = resources[3];

    let u1admin_badge = get_user_balc(&mut test_runner, &user1, admin_badge_addr, "u1admin_badge");

    let u1wd_badge = get_user_balc(&mut test_runner, &user1, wd_badge_addr, "u1wd_badge");
    let u1auth_badge = get_user_balc(&mut test_runner, &user1, auth_badge_addr, "u1auth_badge");

    let u1token = get_user_balc(&mut test_runner, &user1, token_addr, "u1token");

    let u2token = get_user_balc(&mut test_runner, &user2, token_addr, "u2token");

    assert_eq!(u1admin_badge, dec!(3));
    assert_eq!(u1wd_badge, dec!(2));
    assert_eq!(u1auth_badge, dec!(0));
    assert_eq!(u1token, u1token);
    assert_eq!(u2token, u2token);

    println!("-----------------== check vault balances");
    let v1auth_badge = get_vault_balc(
        &mut test_runner,
        compo_addr,
        auth_badge_addr,
        "v1auth_badge",
    );
    assert_eq!(v1auth_badge[0], dec!(1));

    let v1token = get_vault_balc(&mut test_runner, compo_addr, token_addr, "v1token");
    assert_eq!(v1token[0], v1token_exp);

    //----------------== get_vault_data
    /*let data = get_vault_data(&mut test_runner, &user1, compo_addr);
        assert_eq!(data.0, 1);
        assert_eq!(data.2, total_supply_original);
    */

    println!("-----------------== mint_to_bucket");
    let amount = dec!(1000);
    let badge_amount = dec!(3);
    invoke_badge_access_decimal(
        &mut test_runner,
        &user1,
        compo_addr,
        "mint_to_bucket",
        amount,
        admin_badge_addr,
        badge_amount,
    );
    /*let data = get_vault_data(&mut test_runner, &user1, compo_addr);
        assert_eq!(data.2, total_supply + amount);
        total_supply += amount;
        println!("total supply increased accordingly");
    */
    let u1token = get_user_balc(&mut test_runner, &user1, token_addr, "u1token");
    assert_eq!(u1token, amount);
    u1token_exp = u1token;
    println!("user token balance increased accordingly");

    let v1token = get_vault_balc(&mut test_runner, compo_addr, token_addr, "v1token");
    assert_eq!(v1token[0], total_supply_original);
    v1token_exp = total_supply_original;
    println!("vault token balance stays the same");

    println!("-----------------== Sending tokens from user1 to user2");
    let amount = dec!(123);
    send_tokens(
        &mut test_runner,
        &user1,
        &user2,
        compo_addr,
        amount,
        token_addr,
    );
    let u1token = get_user_balc(&mut test_runner, &user1, token_addr, "u1token");
    assert_eq!(u1token, u1token_exp - amount);
    u1token_exp = u1token;

    let u2token = get_user_balc(&mut test_runner, &user2, token_addr, "u2token");
    assert_eq!(u2token, u2token_exp + amount);
    u2token_exp = u2token;
    println!("Successfully sending tokens from user1 to user2");

    println!("-----------------== mint_to_vault");
    let amount = dec!(1000);
    let badge_amount = dec!(3);
    invoke_badge_access_decimal(
        &mut test_runner,
        &user1,
        compo_addr,
        "mint_to_vault",
        amount,
        admin_badge_addr,
        badge_amount,
    );
    let v1token = get_vault_balc(&mut test_runner, compo_addr, token_addr, "v1token");
    assert_eq!(v1token[0], v1token_exp + amount);
    v1token_exp = v1token[0];
    println!("vault amount increased accordingly");

    /*let data = get_vault_data(&mut test_runner, &user1, compo_addr);
        assert_eq!(data.2, total_supply + amount);
        total_supply += amount;
    */

    println!("-----------------== withdraw_to_bucket");
    let amount = dec!(937);
    let badge_amount = dec!(3);
    invoke_badge_access_decimal(
        &mut test_runner,
        &user1,
        compo_addr,
        "withdraw_to_bucket",
        amount,
        admin_badge_addr,
        badge_amount,
    );
    let v1token = get_vault_balc(&mut test_runner, compo_addr, token_addr, "v1token");
    assert_eq!(v1token[0], v1token_exp - amount);
    v1token_exp = v1token[0];
    println!("vault amount decreased accordingly");
    /*let data = get_vault_data(&mut test_runner, &user1, compo_addr);
        assert_eq!(data.2, total_supply);
    */
    let u1token = get_user_balc(&mut test_runner, &user1, token_addr, "u1token");
    assert_eq!(u1token, u1token_exp + amount);
    u1token_exp = u1token;
    println!("u1token increased accordingly");

    println!("-----------------== deposit");
    let amount = dec!(37);
    let badge_amount = dec!(3);
    invoke_badge_access_with_bucket(
        &mut test_runner,
        &user1,
        compo_addr,
        "deposit_to_vault",
        amount,
        token_addr,
        admin_badge_addr,
        badge_amount,
    );
    let v1token = get_vault_balc(&mut test_runner, compo_addr, token_addr, "v1token");
    assert_eq!(v1token[0], v1token_exp + amount);
    v1token_exp = v1token[0];
    println!("vault amount increased accordingly");
    /*let data = get_vault_data(&mut test_runner, &user1, compo_addr);
        assert_eq!(data.2, total_supply);
    */
    let u1token = get_user_balc(&mut test_runner, &user1, token_addr, "u1token");
    assert_eq!(u1token, u1token_exp - amount);
    u1token_exp = u1token;
    println!("u1token decreased accordingly");

    println!("-----------------== burn_in_vault");
    let amount = dec!(100);
    let badge_amount = dec!(3);
    invoke_badge_access_decimal(
        &mut test_runner,
        &user1,
        compo_addr,
        "burn_in_vault",
        amount,
        admin_badge_addr,
        badge_amount,
    );
    let v1token = get_vault_balc(&mut test_runner, compo_addr, token_addr, "v1token");
    assert_eq!(v1token[0], v1token_exp - amount);
    v1token_exp = v1token[0];
    println!("vault amount decreased accordingly");
    /*let data = get_vault_data(&mut test_runner, &user1, compo_addr);
        assert_eq!(data.2, total_supply - amount);
        total_supply -= amount;
    */

    println!("-----------------== burn_in_bucket");
    let amount = dec!(12);
    let badge_amount = dec!(3);
    invoke_badge_access_with_bucket(
        &mut test_runner,
        &user1,
        compo_addr,
        "burn_in_bucket",
        amount,
        token_addr,
        admin_badge_addr,
        badge_amount,
    );
    /*let data = get_vault_data(&mut test_runner, &user1, compo_addr);
        assert_eq!(data.2, total_supply - amount);
        total_supply -= amount;
        assert_eq!(data.1, v1token_exp);
        println!("vault amount decreased accordingly");
    */
    let u1token = get_user_balc(&mut test_runner, &user1, token_addr, "u1token");
    assert_eq!(u1token, u1token_exp - amount);
    u1token_exp = u1token;
    println!("u1token decreased accordingly");
    /*
        println!("-----------------== read_metadata");
        let function_name = "get_token_metadata";
        let txn_receipt = call_function(
            &mut test_runner,
            &user1,
            package_addr,
            blueprint_name,
            function_name,
            token_addr,
            keys_owned.clone(),
        );
        /*let data: (u8, NonFungibleIdType, Decimal, Vec<String>) = txn_receipt;//.output(1);
        println!("call_function output:{:?}\n", data);
        let data_values = vec_option_string(data.3);

        assert_eq!(data.0, 18);
        assert_eq!(data.1, NonFungibleIdType::Integer);
        assert_eq!(data.2, total_supply);
        assert!(
            do_vecs_match(&data_values, &values_owned),
            "token metadata do not match"
        );
        println!("all metadata match accordingly");
    */
        println!("-----------------== update_metadata");
        let badge_amount = dec!(3);
        let key = "name".to_owned();
        let value = "Gold Coin".to_owned();
        update_metadata(
            &mut test_runner,
            &user1,
            compo_addr,
            key.clone(),
            value.clone(),
            admin_badge_addr,
            badge_amount,
        );

        println!("-----------------== read_metadata");
        let function_name = "get_token_metadata";
        let txn_receipt = call_function(
            &mut test_runner,
            &user1,
            package_addr,
            blueprint_name,
            function_name,
            token_addr,
            keys_owned.clone(),
        );
        println!("txn_receipt:{:?}",txn_receipt);
        /*let data: (u8, NonFungibleIdType, Decimal, Vec<String>) = txn_receipt;
        println!("call_function output:{:?}\n", data);
        let data_values = vec_option_string(data.3);

        assert_eq!(data.0, 18);
        assert_eq!(data.1, NonFungibleIdType::Integer);
        assert_eq!(data.2, total_supply);

        let (values_owned, _) = find_replace_two_vec(values_owned, &keys_owned, &key, value.clone());
        println!("new values_owned:{:?}", values_owned);
        assert!(
            do_vecs_match(&data_values, &values_owned),
            "token metadata do not match"
        );
        println!("all metadata match accordingly");
    */
        println!("-----------------== set_token_stage_three");
        let badge_amount = dec!(3);
        invoke_badge_access(
            &mut test_runner,
            &user1,
            compo_addr,
            "set_token_stage_three",
            admin_badge_addr,
            badge_amount,
        );
        */
}
