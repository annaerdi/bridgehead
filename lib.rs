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
            Self { contacts: Mapping::default() }
        }

        #[ink(message)]
        pub fn add_contact(&mut self, account: AccountId, contact: AccountId) {
            self.contacts.insert(&account, &contact);
        }

        #[ink(message)]
        pub fn get_contact(&self, account_id: AccountId) -> Option<AccountId> {
            self.contacts.get(&account_id)
        }
    }
}
