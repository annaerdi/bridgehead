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
            ink::env::debug_println!("created new instance at {}", Self::env().block_number());
            Self { contacts: Mapping::default() }
        }

        #[ink(message)]
        pub fn add_contact(&mut self, account: AccountId, contact: AccountId) {
            self.contacts.insert(account, &contact);
        }

        #[ink(message)]
        pub fn get_contact(&self, account_id: AccountId) -> Option<AccountId> {
            self.contacts.get(&account_id)
        }


        /// Get address for specific name.
        #[ink(message)]
        pub fn get_address(&self, name: AccountId) -> AccountId {
            self.get_address_or_default(name)
        }

        /// Returns the address given the hash or the default address.
        fn get_address_or_default(&self, name: AccountId) -> AccountId {
            self.contacts
                .get(name)
                .unwrap_or(name)
        }


    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn new_creates_empty_phone_book() {
            let phonebook = PhoneBook::new();
            let non_existent_account: AccountId = [0x00; 32].into();
            assert_eq!(phonebook.get_contact(non_existent_account), None);
        }

        #[ink::test]
        fn add_and_get_contact_works() {
            let mut phonebook = PhoneBook::new();
            let account: AccountId = [0x01; 32].into();
            let contact1: AccountId = [0x02; 32].into();
            let contact2: AccountId = [0x02; 32].into();

            println!("{:?}", contact1);


            // Add a contact
            phonebook.add_contact(account, contact1);
            phonebook.add_contact(account, contact2);

            println!("{:?}", phonebook.get_contact(account));

            // Now, the contact should be retrievable
            assert_eq!(phonebook.get_contact(account), Some(contact1));
            assert_eq!(phonebook.get_contact(account), Some(contact2)); // if i dont put some around it, i get the error that it expected `Option<AccountId>`, found `AccountId`
        }
    }
}
