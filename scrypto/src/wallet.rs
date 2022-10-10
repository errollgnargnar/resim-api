use scrypto::prelude::*;

blueprint! {
    /*
    Shared wallet structure: vault_account stores all token vaults and deposit_whitelist stores the permissioned external addresses
    that can deposit to the wallet.
    */
    struct Treasury {
        vault_account: HashMap<ResourceAddress, Vault>,
        deposit_whitelist: HashMap<String, Vec<String>>,
        // Admin mint badge is used to mint the admin badge. 
        admin_mint_badge_vault: Vault,
        // Admin badge has authority to mint or burn depositor badges as they are added or removed from the whitelist.
        // This badge will be attached to the add and remove from whitelist methods.
        admin_badge: ResourceAddress, 
        // Depositor badge is attached to the addresses in the whitelist. Gives authority to deposit to the wallet.
        depositor_badge: ResourceAddress,
        // Instead of burning the badge, try and recall from depositor wallet to treassury wallet and then burn.
    
    }
    // Here we are implimenting the shared wallet struct and giving it the new function to allow for instantiation. 
    // The two elements of the struct start off as empty HashMaps. 
    impl Treasury {
      
        pub fn new() -> (ComponentAddress, Bucket) {
            let admin_mint_badge: Bucket = ResourceBuilder::new_fungible()
            .metadata("name", "admin_mint_badge")
            //.restrict_deposit(rule!(deny_all), LOCKED)
            .restrict_withdraw(rule!(deny_all), LOCKED)
            .divisibility(DIVISIBILITY_NONE)
            .initial_supply(1); 

            let admin_badge: ResourceAddress = ResourceBuilder::new_fungible()
            .metadata("name", "admin_badge")
            .mintable(rule!(require(admin_mint_badge.resource_address())), LOCKED)
            //.restrict_deposit(rule!(deny_all), LOCKED)
            .restrict_withdraw(rule!(deny_all), LOCKED)
            .divisibility(DIVISIBILITY_NONE)
            .no_initial_supply(); 

            let admin_badge_resource_manager: &mut ResourceManager = borrow_resource_manager!(admin_badge);
            let first_admin_badge = admin_mint_badge.authorize(|| admin_badge_resource_manager.mint(1));

            

            let depositor_badge: ResourceAddress = ResourceBuilder::new_fungible()
            .metadata("name", "depositor_badge")
            .mintable(rule!(require(admin_badge)), LOCKED)
            .burnable(rule!(require(admin_badge)), LOCKED)
            .restrict_deposit(rule!(require(admin_badge)), LOCKED)
            .restrict_withdraw(rule!(require(admin_badge)), LOCKED)
            .divisibility(DIVISIBILITY_NONE)
            .no_initial_supply();


            let rules: AccessRules = AccessRules::new()
            .method("add_to_whitelist", rule!(require(admin_badge)))
            // .method("mint_and_take_depositor_badge", rule!(require(admin_badge)))
            // .method("burn_depositor_badge", rule!(require(admin_badge)))
            .method("remove_from_whitelist", rule!(require(admin_badge)))
            .method("add_token", rule!(require(admin_badge)))
            .method("remove_token", rule!(require(admin_badge)))
            .method("deposit", rule!(require(depositor_badge)))
            .default(rule!(allow_all));

            let mut treasury_component: TreasuryComponent = Self {
                admin_mint_badge_vault: Vault::with_bucket(admin_mint_badge),
                admin_badge: admin_badge,
                depositor_badge: depositor_badge,
                vault_account: HashMap::new(),
                deposit_whitelist: HashMap::new(),
            
            }.instantiate();
            treasury_component.add_access_check(rules);
            (treasury_component.globalize(), first_admin_badge)            
        }
    
        // This method is used to check the balance of a desired tokken vault.
        pub fn balances(&self, token_resource_id: ResourceAddress) {
            info!("My token balance is {:?}", self.vault_account.get(&token_resource_id)); // This is the line that prints the balance of the desired token.
        }

        pub fn deposit(&mut self, token_resource_id: ResourceAddress, bucket: Bucket) {
            // This method is used to deposit tokens into the desired token vault.
            self.vault_account.get_mut(&token_resource_id).unwrap().put(bucket); // This is the line that deposits tokens into the desired token vault.
            // Work in progress. Method protected by the depositor_badge accessrule.
        }

        pub fn withdraw(&mut self, token_resource_id: ResourceAddress, amount: Decimal) {
            // This method is used to withdraw tokens from the desired token vault.
            self.vault_account.get_mut(&token_resource_id).unwrap().take(amount); // This is the line that withdraws tokens from the desired token vault.
            // Work in progress. Needs to integrate functionality with a type of multisig or other mechanism of governance & security.
            // Look into using the "require" function to require a certain number of signatures from a group of addresses.
            // Look into on-chain governance as an alternative to multisig.
        }

        // This method is used to add a token vault to the vault_account HashMap using the ResourceAddress of the desired token. (WORKS)
        pub fn add_token(&mut self, token_resource_id: ResourceAddress) {
            self.vault_account.insert(token_resource_id, Vault::new(token_resource_id)); // This is the line that adds a new vault to the vault_account HashMap.
        }

        // This method is used to remove a token vault from the vault_account HashMap using the token name (Key).
        pub fn remove_token(&mut self, token_resource_id: ResourceAddress) {
            self.vault_account.remove(&token_resource_id); // This is the line that removes a vault from the vault_account HashMap.
        }

        // This method is used to add an external address to the deposit_whitelist HashMap using the entity name (Key) and the external address (Value). (WORKS)
        pub fn add_to_whitelist(&mut self, entity_name: String, address: ComponentAddress) {
            self.deposit_whitelist.entry(entity_name).or_default().push(address.to_string()); // This is the line that adds an external address to the deposit_whitelist HashMap.
            
        }

        // This method is used to mint and take the depositor_badge from the Treasury and transfer it to the address in the deposit_whitelist HashMap.
        // pub fn mint_and_take_depositor_badge(&mut self, entity_name: String, address: ComponentAddress) {
        //     // Note: the address in the deposit_whitelist is a String type.
        //     //let user_address = Runtime::actor().as_component();
        //     let match_address = address.to_string(); /*
        //     This is the address that is being checked against the address in the deposit_whitelist HashMap.
        //     Possible Solution: Find a way to remove "self" from deposit_whitelist_address and then use the "require" function to check if the address in 
        //     the deposit_whitelist HashMap matches the address in the deposit_whitelist HashMap. (TENTATIVE)
        //     */
        //     // Another possible solution is to utilize the "proof" in scrypto docs
        //     // Might need to change function such that it does not take in an address as a parameter, but rather the address of the account interacting with it.
        //     // If kept the same, someone may just put in an address that is in the deposit_whitelist HashMap and mint a badge for themselves.
            
        //     //.require(self.deposit_whitelist.get(&entity_name).unwrap().contains(&match_address)); // This is the line that checks if the address in the deposit_whitelist HashMap matches the address in the deposit_whitelist HashMap.
        //     //match_address == self.deposit_whitelist.get(&entity_name).unwrap()[0]; // Not sure what the [0] is doing here. Need to test if it is only comparing the first element in the String.
        //     //let deposit_whitelist_address = self.deposit_whitelist.get(&entity_name); // This is the address in the deposit_whitelist HashMap.
        //     match match_address { 
        //         y if y == self.deposit_whitelist.get(&entity_name).unwrap()[0] => { /*
        //             This is the line that checks if the address in the deposit_whitelist HashMap matches 
        //             the address in the deposit_whitelist HashMap.
        //             Need to create authorization for the depositor_badge to be transferred to the address in the deposit_whitelist HashMap.
        //             */
                    
        //             let admin_badge_resource_manager: &mut ResourceManager = borrow_resource_manager!(self.depositor_badge);
        //             // Issue: The following line is not working. Need to figure out how to authorize the minting of the depositor_badge.
        //             let mut auth_and_mint: Bucket = self.admin_badge.authorize(|| admin_badge_resource_manager.mint(1));
        //                 auth_and_mint.take(1);
                    
        //         }
        //         _ => {
        //             info!("The address does not match the address in the deposit_whitelist HashMap.");
        //         }
        //     }
        // }

        // This method is used to remove an external address from the deposit_whitelist HashMap using the entity name (Key). (WORKS)
        pub fn remove_from_whitelist(&mut self, entity_name: String) { 
            self.deposit_whitelist.remove(&entity_name); // This is the line that removes the key and value from the deposit_whitelist HashMap.
        }

        // This method is used to burn the depositor_badge from the address that is not in the deposit_whitelist HashMap. 
        // Need to update this method once "recall" is available.
        // At this point, this method requires the depositor to initiate the burn of the depositor_badge.
        // pub fn burn_depositor_badge(&mut self, entity_name: String, depositor_badge_with_bucket: Bucket, address: ComponentAddress) {
        //     //let user_address = Runetime::get_caller().as_component();
        //     let match_address = address.to_string(); // This is the address that is being checked against the address in the deposit_whitelist HashMap.
        //     //let deposit_whitelist_address = self.deposit_whitelist.get(&entity_name); // This is the address in the deposit_whitelist HashMap.
        //     match match_address { 
        //         y if y == self.deposit_whitelist.get(&entity_name).unwrap()[0] => { /*
        //             This is the line that checks if the address in the deposit_whitelist HashMap matches 
        //             the address in the deposit_whitelist HashMap.
        //             */
        //             info!("The address matches the address in the deposit_whitelist HashMap.");
        //         }
        //         _ => {
        //             // Need to create authorization for the depositor_badge to be burned.
        //             let admin_badge_resource_manager: &mut ResourceManager = borrow_resource_manager!(self.depositor_badge);
        //             // Issue: The following line is not working. Need to figure out how to authorize the burning of the depositor_badge.
        //             let auth_and_burn = self.admin_mint_badge.authorize(|| admin_badge_resource_manager.burn(depositor_badge_with_bucket));
        //                 return auth_and_burn
                        
                    
        //         }
        //     }
        // }

        // This method is used to query the address of the component. (WORKS)
        pub fn request_component_address(&self) {
            let address = Runtime::actor().as_component(); // This is the line that requests the address of the component.
            info!("Treasury Component Address: {:?}", address); // This is the line that prints the address of the component. 
        }
    }
}