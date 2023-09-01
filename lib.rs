#![cfg_attr(not(feature = "std"), no_std)]

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
    }

}
