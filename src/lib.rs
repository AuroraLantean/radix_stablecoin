use scrypto::prelude::*;

// credit: thanks to Scrypto-Example/regulated-token
// admin, version, withdraw, mint, burn,
blueprint! {
    struct StableCoinVault {
        token_vault: Vault,
        auth: Vault,
        total_supply: Decimal,
        version: u16,
        admin_addr: ResourceAddress,
    }

    impl StableCoinVault {
        pub fn new(total_supply: Decimal) -> (ComponentAddress, Bucket, Bucket) {
          info!("StableCoin new(): total_supply = {}", total_supply);
            // top admin
            let admin_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "admin_badge")
                .burnable(rule!(allow_all), LOCKED)
                .initial_supply(3);
            // to withdraw coins
            let wd_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "withdraw badge")
                .burnable(rule!(allow_all), LOCKED)
                .initial_supply(2);

            // for minting & withdraw authority
            let auth_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "auth_badge")
                .burnable(rule!(allow_all), LOCKED)
                .initial_supply(1);
            info!("check1");

            let token_rule: AccessRule = rule!(
                require(admin_badge.resource_address())
                    || require(auth_badge.resource_address())
            );
            info!("check2");
            let my_bucket: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_MAXIMUM)
                .metadata("name", "USD Tether")
                .metadata("symbol", "USDT")
                .metadata("icon_url", "https://token_website.com/icon.ico")
                .metadata("url", "https://token_website.com")
                .metadata("creator", "Xman")
                .metadata(
                    "version",
                    "version 1 - Fixed supply, withdraw may be restricted",
                )
                .updateable_metadata(token_rule.clone(), token_rule.clone())
                //.restrict_withdraw(token_rule.clone(), token_rule.clone())
                .mintable(token_rule.clone(), token_rule.clone())
                .burnable(token_rule.clone(), token_rule.clone())
                .initial_supply(total_supply);
            info!("check3");
            // Next we need to setup the access rules for the methods of the component
            let method_rule: AccessRules = AccessRules::new()
                .method(
                    "get_data",
                    rule!(allow_all), AccessRule::DenyAll
                )
                .default(token_rule, AccessRule::DenyAll);
            info!("check4");
            let mut component = Self {
                token_vault: Vault::with_bucket(my_bucket),
                auth: Vault::with_bucket(auth_badge),
                total_supply,
                version: 1,
                admin_addr: admin_badge.resource_address(),
            }
            .instantiate();
            component.add_access_check(method_rule);
            info!("check5");
            (component.globalize(), admin_badge, wd_badge)
        }// new()

        pub fn mint_to_bucket(&mut self, amount: Decimal) -> Bucket {
          info!("mint_to_bucket");
          assert!(amount > dec!(0), "invalid amount");
          info!("self.total_supply:{}", self.total_supply);
          self.total_supply += amount;
          info!("self.total_supply:{}", self.total_supply);
          self.auth.authorize(|| {
            borrow_resource_manager!(self.token_vault.resource_address()).mint(amount)
          })
        }
        pub fn mint_to_vault(&mut self, amount: Decimal) {
          info!("mint_to_vault");
          assert!(amount > dec!(0), "invalid amount");
          let new_tokens = borrow_resource_manager!(self.token_vault.resource_address())
          .mint(amount);
          self.token_vault.put(new_tokens);
          self.total_supply += amount;
          info!("total token amount: {}", self.token_vault.amount());
        }

        //pub fn withdraw_to_3rd_party(&self, amount: Decimal) {
          //risky... just send tokens to yourself, then deposit them into the 3rd party package!
        //}
        pub fn withdraw(&mut self, amount: Decimal) -> Bucket {
          info!("withdraw_from_vault");
          //check set_withdrawable_vault()
          assert!(amount > dec!(0), "invalid amount");
          assert!(amount <= self.token_vault.amount(), "not enough amount in the vault");
          self.token_vault.take(amount)
        }
        pub fn deposit_to_vault(&mut self, bucket: Bucket) {
          self.token_vault.put(bucket);
        }

        pub fn burn_in_vault(&mut self, amount: Decimal) {
          info!("burn_in_vault");
          assert!(amount > dec!(0), "invalid amount");
          assert!(amount <= self.token_vault.amount(), "not enough amount in the vault");
          self.total_supply -= amount;
          self.token_vault.take(amount).burn();
        }
        pub fn burn_in_bucket(&mut self, mut bucket: Bucket) {
          info!("burn_in_bucket");
          assert!(bucket.resource_address() == self.token_vault.resource_address(), "input token invalid");
          let amount = bucket.amount();
          self.total_supply -= amount;
          bucket.take(amount).burn();
        }

        pub fn get_data(&self) -> (u16, Decimal, Decimal) {
            let amount = self.token_vault.amount();
            info!("Current version: {}, vault_amount: {}, total_supply:{}", self.version, amount, self.total_supply );
            (self.version, amount, self.total_supply)
        }
        pub fn set_version(&mut self, new_version: u16) {
            info!("Current version is {}", self.version);
            assert!(self.version >= 3, "invalid version");
            self.version = new_version;
        }
    }
}
/*
                .method(
                    "set_withdrawable_vault",
                    rule!(
                        require(admin_badge.resource_address())
                            || require(wd_badge.resource_address())
                    ), AccessRule::DenyAll
                )
                .method(
                    "lift_restriction",
                    rule!(require(admin_badge.resource_address())), AccessRule::DenyAll
                )

        /// Either the general admin or withdraw badge may be used to seal withdrawing tokens from the vault
        pub fn set_withdrawable_vault(&self, is_withdrawable: bool) {
            // this function will fail if version >= 3 and the token behavior has been locked
            let token_rmgr: &mut ResourceManager =
                borrow_resource_manager!(self.token_vault.resource_address());

            self.auth.authorize(|| {
                if is_withdrawable {
                    token_rmgr.set_withdrawable(rule!(
                        require(self.admin_addr)
                            || require(self.auth.resource_address())
                    ));
                    info!("Token withdraw is now RESTRICTED");
                } else {
                    token_rmgr.set_withdrawable(rule!(allow_all));
                    info!("Token withdraw is lifted");
                }
            })
        }

        pub fn lift_restriction(&mut self) {
            // Adding the auth badge to the component auth zone to allow for the operations below
            info!("lift_restriction...");
            ComponentAuthZone::push(self.auth.create_proof());

            assert!(self.version <= 2, "Already at version > 2");
            let token_rmgr: &mut ResourceManager =
                borrow_resource_manager!(self.token_vault.resource_address());

            info!("version: {}", self.version);
            if self.version == 1 {
              info!("check11");
              self.version = 2;
                token_rmgr.set_metadata("version".into(), "version 2 - Unlimited supply, may be restricted withdraw".into());
                // set token minting to only auth
                token_rmgr
                    .set_mintable(rule!(require(self.auth.resource_address())));
                info!("check12");

                // Drop the last added proof to the component auth zone
                ComponentAuthZone::pop().drop();
                info!("check13");
            } else {
                self.version = 3;
                info!("check21");
                // Update token's metadata to reflect the final version
                token_rmgr.set_metadata("version".into(), "version 3 - fixed supply".into());

                info!("check22");
                // Removing restricted withdraw and minting
                //token_rmgr.set_mintable(rule!(deny_all));
                token_rmgr.set_withdrawable(rule!(allow_all));
                token_rmgr.set_updateable_metadata(rule!(deny_all));

                info!("check23");
                // Permanently fix the token behavior
                token_rmgr.lock_mintable();
                token_rmgr.lock_withdrawable();
                token_rmgr.lock_updateable_metadata();

                // With the resource behavior forever locked, our auth no longer has any use
                // We will burn our auth badge, and the holders of the other badges may burn them at will
                // Our badge has the allows everybody to burn, so there's no need to provide a burning authority

                // Drop the last added proof to the component auth zone
                info!("check24");
                ComponentAuthZone::pop().drop();
                self.auth.take_all().burn();
                info!("check25");
            }
        }
*/
