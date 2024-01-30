#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod phonebook_contract {
    use ink::storage::Mapping;
    use ink_prelude::vec::Vec;


    #[ink(storage)]
    pub struct PhoneBook {
        contacts: Mapping<AccountId, Vec<AccountId>>,
    }

    impl PhoneBook {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink::env::debug_println!("created new instance at {}", Self::env().block_number());
            Self { contacts: Mapping::default() }
        }

        /// Let a user add new contact to their contact list
        #[ink(message)]
        pub fn add_contact(&mut self, account: AccountId, contact: AccountId) {
            let mut contacts = self.contacts.get(&account).unwrap_or_default();
            contacts.push(contact);
            self.contacts.insert(account, &contacts);
        }

        /// Get the list of contacts of a user
        #[ink(message)]
        pub fn get_contacts(&self, account: AccountId) -> Vec<AccountId> {
            self.contacts.get(&account).unwrap_or_default()
        }

    }


    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn add_and_get_contacts_works() {
            let mut phonebook = PhoneBook::new();
            let account: AccountId = [0x01; 32].into();
            let contact1: AccountId = [0x02; 32].into();
            let contact2: AccountId = [0x03; 32].into();

            // Add two contacts to the same account
            phonebook.add_contact(account, contact1);
            phonebook.add_contact(account, contact2);

            // Retrieve the contacts for the account
            let retrieved_contacts = phonebook.get_contacts(account);
            println!("{:?}", retrieved_contacts);

            // Check that both contacts are in the retrieved list
            assert_eq!(retrieved_contacts, vec![contact1, contact2]);
        }


    }

}