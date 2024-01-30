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

        #[ink(message)]
        pub fn add_contact(&mut self, account: AccountId, contact: AccountId) {
            let mut contacts = self.contacts.get(&account).unwrap_or_default();
            contacts.push(contact);
            self.contacts.insert(account, &contacts);
        }


        #[ink(message)]
        pub fn get_contacts(&self, account: AccountId) -> Vec<AccountId> {
            self.contacts.get(&account).unwrap_or_default()
        }

    }


    // #[ink(message)]
    //     pub fn get_contact(&self, account_id: AccountId) -> Option<AccountId> {
    //         self.contacts.get(&account_id)
    //     }
    //
    //
    //     /// Get address for specific name.
    //     #[ink(message)]
    //     pub fn get_address(&self, name: AccountId) -> AccountId {
    //         self.get_address_or_default(name)
    //     }
    //
    //     /// Returns the address given the hash or the default address.
    //     fn get_address_or_default(&self, name: AccountId) -> AccountId {
    //         self.contacts
    //             .get(name)
    //             .unwrap_or(name)
    //     }




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


        // #[ink::test]
        // fn new_creates_empty_phone_book() {
        //     let phonebook = PhoneBook::new();
        //     let non_existent_account: AccountId = [0x00; 32].into();
        //     assert_eq!(phonebook.get_contact(non_existent_account), None);
        // }

        // #[ink::test]
        // fn add_and_get_contact_works() {
        //     let mut phonebook = PhoneBook::new();
        //     let account: AccountId = [0x01; 32].into();
        //     let contact1: AccountId = [0x02; 32].into();
        //     let contact2: AccountId = [0x03; 32].into();
        //
        //     // Add two contacts to the same account
        //     phonebook.add_contact(account, contact1);
        //     phonebook.add_contact(account, contact2);
        //
        //     // Retrieve the contact for the account
        //     let retrieved_contact = phonebook.get_contacts(account);
        //
        //     // Check that the retrieved contact is the last one added
        //     assert_eq!(retrieved_contact, Some(contact2));
        //
        //     // The first contact (contact1) should have been overwritten by contact2
        // }

    }

}