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

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn new_creates_empty_phone_book() {
            let phonebook = PhoneBook::new();
            let account: AccountId = AccountId::from([0xFF as u8; 32]);
            // Initially, there should be no contact for this account
            assert_eq!(phonebook.get_contact(account), None);
        }

        #[ink::test]
        fn add_and_get_contact_works() {
            let mut phonebook = PhoneBook::new();
            let account: AccountId = [0x01; 32].into();
            let contact: AccountId = [0x02; 32].into();

            // Add a contact
            phonebook.add_contact(account, contact);

            // Now, the contact should be retrievable
            assert_eq!(phonebook.get_contact(account), Some(contact));
        }
    }

}
