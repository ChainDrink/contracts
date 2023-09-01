#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod chaindrink {
    #[ink(storage)]
    pub struct ChainDrink {
        owner: AccountId,
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
            Self { owner: init_owner }
        }

        #[ink(message)]
        pub fn owner(&self) -> AccountId {
            self.owner
        }

        #[ink(message, payable)]
        pub fn receive(&mut self) {
            assert!(self.env().transferred_value() > 0);
            self.env().emit_event(Received {
                from: Some(self.env().caller()),
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

}
