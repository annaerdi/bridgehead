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
            self.contacts.insert(&account, &contact);
        }

        #[ink(message)]
        pub fn get_contact(&self, account_id: AccountId) -> AccountId {
            self.contacts.get(&account_id).unwrap()
        }

        #[ink(message)]
        fn print(&self) {
            let caller = self.env().caller();
            let message = ink_prelude::format!("got a call from {:?}", caller);
            ink::env::debug_println!(&message);
        }


    }

    #[cfg(test)]
    mod tests {
        use super::*;

        // #[ink::test]
        // fn new_creates_empty_phone_book() {
        //     let phonebook = PhoneBook::new();
        //     let account: AccountId = AccountId::from([0xFF as u8; 32]);
        //     // Initially, there should be no contact for this account
        //     assert_eq!(phonebook.get_contact(account), Option<_>);
        // }

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
            assert_eq!(phonebook.get_contact(account), contact1);
            assert_eq!(phonebook.get_contact(account), contact2);
        }
    }

}
