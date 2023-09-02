#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod chaindrink {
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct ChainDrink {
        owner: AccountId,
        balances: Mapping<AccountId, Balance>,
    }

    #[ink(event)]
    pub struct Received {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        value: Balance,
    }

    impl ChainDrink {
        #[ink(constructor)]
        pub fn new(init_owner: AccountId) -> Self {
            let balances = Mapping::default();
            Self { owner: init_owner, balances }
        }

        #[ink(message)]
        pub fn get_balance(&self) -> Option<Balance> {
            let caller = self.env().caller();
            self.balances.get(caller)
        }

        #[ink(message)]
        pub fn owner(&self) -> AccountId {
            self.owner
        }

        #[ink(message, payable)]
        pub fn receive(&mut self) {
            assert!(self.env().transferred_value() > 0);
            let caller = self.env().caller();

            let balance = self.balances.get(caller).unwrap_or(0);
            let endowment = self.env().transferred_value();
            self.balances.insert(caller, &(balance + endowment));

            self.env().emit_event(Received {
                from: Some(caller),
                value: self.env().transferred_value(),
            });
        }

        #[ink(message)]
        pub fn withdraw(&mut self, amount: Balance) {
            assert_eq!(self.env().caller(), self.owner, "Only the owner can withdraw");
            
            assert!(self.env().balance() >= amount, "Not enough balance to withdraw");
            
            let result = self.env().transfer(self.owner, amount);
            assert_eq!(result, Ok(()), "Transfer failed");
        }

    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn default_accounts() -> ink_env::test::DefaultAccounts<ink_env::DefaultEnvironment> {
            ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
        }
    
        #[ink::test]
        fn constructor_works() {
            let accounts = default_accounts();
            let chaindrink = ChainDrink::new(accounts.alice);
            assert_eq!(chaindrink.owner(), accounts.alice);
        }

        #[ink::test]
        fn receive_works() {
            let accounts = default_accounts();
            let mut chaindrink = ChainDrink::new(accounts.alice);
            
            let initial_balance = chaindrink.get_balance().unwrap_or(0);
            assert_eq!(initial_balance, 0);

            let endowment = 50;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(endowment);
            chaindrink.receive();
    
            let new_balance = chaindrink.get_balance().unwrap_or(0);
            assert_eq!(new_balance, 50);
        }

    }

}
