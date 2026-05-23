#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod real_contract {
    #[ink(storage)]
    pub struct RealContract {
        value: bool,
    }

    impl RealContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { value: false }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new()
        }

        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }
}
