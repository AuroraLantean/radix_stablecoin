use scrypto::prelude::*;
// credit: thanks to Scrypto-Example/regulated-token
// admin, version, withdraw, mint, burn,

//pub mod gumball_club;//to import gumball_club.rs
//pub mod gumball_machine;

/* #[derive(ScryptoSbor, NonFungibleData)]
pub struct RoyaltyShare {
    pub name: String,
    #[mutable]
    pub available: bool,
    pub account_component: ComponentAddress,
    pub percentage: Decimal,
} */
#[blueprint]
mod stable_coin_vault {
    enable_method_auth! {
        roles {
            super_admin => updatable_by: [OWNER];
            admin => updatable_by: [super_admin, OWNER];
        },
        methods {
            free_token => PUBLIC;
            buy => PUBLIC;
            change_price => restrict_to: [admin];
            mint_to_bucket => restrict_to: [admin];
            mint_to_vault => restrict_to: [admin];
            withdraw_to_bucket => restrict_to: [OWNER];
            deposit_to_vault => PUBLIC;
            burn_in_vault => restrict_to: [super_admin];
            burn_in_bucket => restrict_to: [admin];
            update_metadata => restrict_to: [admin];
            set_token_stage_three => restrict_to: [super_admin, OWNER];
            //redeem_profits => restrict_to: [super_admin, OWNER];
            get_vault_data => PUBLIC;
            set_version => restrict_to: [super_admin];
        }
    }
    struct StableCoinVault {
        token_vault: Vault,
        //auth: Vault,
        total_supply: Decimal,
        version: u16,
        //admin_addr: ResourceAddress,
    }
    impl StableCoinVault {
        pub fn new(
            total_supply: Decimal,
            keys: Vec<String>,
            values: Vec<String>,
            owner_badge_addr: ResourceAddress,
        ) -> (Global<StableCoinVault>, Bucket, Bucket) {
            info!("StableCoin new(): total_supply = {}", total_supply);

            // super_admin
            let super_admin_badge: FungibleBucket = ResourceBuilder::new_fungible(OwnerRole::None)
                .divisibility(DIVISIBILITY_NONE)
                .metadata(metadata! {
                  init {
                      "name" => "super_admin", locked;
                  }
                })
                .mint_initial_supply(2);

            // admin_badge
            let admin_badge: FungibleBucket = ResourceBuilder::new_fungible(OwnerRole::None)
                .divisibility(DIVISIBILITY_NONE)
                .metadata(metadata! {
                  init {
                      "name" => "admin_badge", locked;
                  }
                })
                .mint_initial_supply(1);

            let my_bucket: FungibleBucket = ResourceBuilder::new_fungible(OwnerRole::None)
                .divisibility(DIVISIBILITY_MAXIMUM)
                .metadata(metadata! {
                    init {
                      keys[0].as_str() => values[0].as_str(), locked;
                      keys[1].as_str() => values[1].as_str(), locked;
                      keys[2].as_str() => values[2].as_str(), locked;
                      keys[3].as_str() => values[3].as_str(), locked;
                      keys[4].as_str() => values[4].as_str(), locked;
                      keys[5].as_str() => values[5].as_str(), locked;
                    }
                })
                .mint_initial_supply(total_supply);

            info!("check2");
            let component = Self {
                token_vault: Vault::with_bucket(my_bucket.into()),
                total_supply,
                version: 1,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::Fixed(rule!(require(owner_badge_addr))))
            .roles(roles!(
                super_admin => rule!(require_amount(dec!(2),super_admin_badge.resource_address()));
                admin => rule!(require(admin_badge.resource_address()));
            ))
            .globalize();
            //OwnerRole::None
            return (component, super_admin_badge.into(), admin_badge.into());
        }

        pub fn free_token(&mut self) -> Bucket {
            info!("balance: {}", self.token_vault.amount());
            self.token_vault.take(1)
        }
        pub fn buy(&mut self, _funds: Bucket) -> Bucket {
            self.token_vault.take(1)
        }

        pub fn change_price(&mut self, _new_price: Decimal) {
            // -- snip --
        }

        pub fn mint_to_bucket(&mut self, amount: Decimal) -> Bucket {
            info!("mint_to_bucket");
            assert!(amount > dec!(0), "invalid amount");
            info!("self.total_supply:{}", self.total_supply);
            self.total_supply += amount;
            info!("self.total_supply:{}", self.total_supply);
            //self.auth.authorize(|| {
            ResourceManager::from_address(self.token_vault.resource_address()).mint(amount)
        }
        pub fn mint_to_vault(&mut self, amount: Decimal) {
            info!("mint_to_vault");
            assert!(amount > dec!(0), "invalid amount");
            let new_tokens =
                ResourceManager::from_address(self.token_vault.resource_address()).mint(amount);
            self.token_vault.put(new_tokens);
            self.total_supply += amount;
            info!("total token amount: {}", self.token_vault.amount());
        }

        /*pub fn withdraw_to_3rd_party(&self, amount: Decimal) {
        // risky... just send tokens to yourself, then deposit them into the 3rd party package!
        }*/

        pub fn withdraw_to_bucket(&mut self, amount: Decimal) -> Bucket {
            info!("withdraw_to_bucket");
            //check set_withdrawable_vault()
            assert!(amount > dec!(0), "invalid amount");
            assert!(
                amount <= self.token_vault.amount(),
                "not enough amount in the vault"
            );
            self.token_vault.take(amount)
        }
        pub fn deposit_to_vault(&mut self, bucket: Bucket) {
            self.token_vault.put(bucket);
        }

        pub fn burn_in_vault(&mut self, amount: Decimal) {
            info!("burn_in_vault");
            assert!(amount > dec!(0), "invalid amount");
            assert!(
                amount <= self.token_vault.amount(),
                "not enough amount in the vault"
            );
            self.total_supply -= amount;
            self.token_vault.take(amount).burn();
        }
        pub fn burn_in_bucket(&mut self, bucket: Bucket) {
            info!("burn_in_bucket");
            assert!(
                bucket.resource_address() == self.token_vault.resource_address(),
                "input token invalid"
            );
            let amount = bucket.amount();
            self.total_supply -= amount;
            bucket.burn();
        }

        // name, symbol, icon_url, url, author, stage
        pub fn update_metadata(&mut self, key: String, value: String) {
            info!("update_metadata");
            ResourceManager::from_address(self.token_vault.resource_address())
                .set_metadata(key, value);
        } //self.auth.authorize(|| {})

        pub fn set_token_stage_three(&self) {
            info!("set_token_stage_three");
            let token_rmgr: ResourceManager =
                ResourceManager::from_address(self.token_vault.resource_address());

            token_rmgr.set_metadata(
                "stage".to_owned(),
                "Lock mint, withdraw, and update_metadata rules".to_owned(),
            );
            //token_rmgr.set_withdrawable(rule!(allow_all));
            //token_rmgr.lock_withdrawable();

            token_rmgr.set_mintable(rule!(deny_all));
            token_rmgr.lock_mintable();

            token_rmgr.set_updatable_metadata(rule!(deny_all));
            token_rmgr.lock_updatable_metadata();

            // With the resource behavior forever locked, our auth no longer has any use
            // We will burn our auth badge, and the holders of the other badges may burn them at will
            // Our badge has the allows everybody to burn, so there's no need to provide a burning authority
        }

        //["name", "symbol", "icon_url", "url", "author", "stage"]
        pub fn get_token_metadata(
            token_addr: ResourceAddress,
            keys_owned: Vec<String>,
        ) -> (u8, NonFungibleIdType, Decimal, Vec<String>) {
            let manager: ResourceManager = ResourceManager::from_address(token_addr);

            let mut divisibility: u8 = 255;
            let mut non_fungible_id_type = NonFungibleIdType::Integer;
            match manager.resource_type() {
                ResourceType::Fungible { divisibility: div } => {
                    info!("Fungible resource with divisibility {}", div);
                    divisibility = div;
                }
                ResourceType::NonFungible {
                    id_type: nft_id_type,
                } => {
                    info!(
                        "Non Fungible resource found with id_type: {:?}",
                        nft_id_type
                    );
                    non_fungible_id_type = nft_id_type;
                }
            }

            let total_supply = manager.total_supply().unwrap_or_default();
            info!("Total supply: {}", total_supply);

            /* fn get_metadata<K: ToString, V: MetadataVal>(&self, name: K) -> Result<Option<V>, MetadataConversionError> {
                self.metadata().get(name)
            }*/
            let mut values: Vec<String> = vec![];
            /*if keys_owned.len() > 0 {
                let value0 = manager
                    .get_metadata(keys_owned[0].clone())
                    .unwrap_or_else(|e| Some("MetadataConversioniError".to_owned()));
                let value00 = value0.unwrap_or_else(|| "err".to_owned());
                values.push(value00)
            }*/

            let mut value_op: Option<String>;
            let mut value: String;
            for key in keys_owned {
                value_op = manager
                    .get_metadata(key)
                    .unwrap_or_else(|_e| Some("MetadataConversioniError".to_owned()));
                value = value_op.unwrap_or_else(|| "err".to_owned());
                values.push(value)
                //.map_or("no_value".to_owned(), |v| v);
            }
            (divisibility, non_fungible_id_type, total_supply, values)
        }

        pub fn get_vault_data(&self) -> (u16, Decimal, Decimal) {
            let amount = self.token_vault.amount();
            info!(
                "Current version: {}, vault_amount: {}, total_supply:{}",
                self.version, amount, self.total_supply
            );
            (self.version, amount, self.total_supply)
        }
        pub fn set_version(&mut self, new_version: u16) {
            info!("Current version is {}", self.version);
            assert!(self.version >= 3, "invalid version");
            self.version = new_version;
        }
    }
}
