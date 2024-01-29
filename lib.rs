#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod phonebook_contract {
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct PhoneBook {
        contacts: Mapping<AccountId, AccountId>,
    }

    impl PhoneBook {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut contacts = Mapping::default();
            let caller = Self::env().caller();
            contacts.insert(&caller, &caller);
            Self {
                contacts,
            }
        }

        #[ink(message)]
        pub fn add_contact(&mut self, contact: AccountId) {
            let caller = self.env().caller();

            self.contacts.insert(caller, &contact);
        }

        #[ink(message)]
        pub fn get_contact(&self, account_id: AccountId) -> Option<AccountId> {
            self.contacts.get(&account_id)
        }

    }
}
