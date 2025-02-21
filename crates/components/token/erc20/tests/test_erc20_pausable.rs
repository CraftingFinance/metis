// Copyright 2018-2021 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg_attr(not(feature = "std"), no_std)]

use ink_prelude::string::String;

#[metis_lang::contract]
pub mod erc20pausable {
    use super::String;
    use erc20::Result;
    use metis_lang::{
        import,
        metis,
    };

    use metis_erc20 as erc20;
    use metis_ownable as ownable;
    use metis_pausable as pausable;

    /// A simple ERC-20 contract.
    #[ink(storage)]
    #[import(ownable, pausable, erc20)]
    pub struct Erc20Pausable {
        ownable: ownable::Data<Erc20Pausable>,
        erc20: erc20::Data<Erc20Pausable>,
        pausable: pausable::Data,
    }

    // impl transfer hook
    impl erc20::pausable::Impl<Erc20Pausable> for Erc20Pausable {}

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    #[metis(erc20)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    /// Event emitted when an approval occurs that `spender` is allowed to withdraw
    /// up to the amount of `value` tokens from `owner`.
    #[ink(event)]
    #[metis(erc20)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    /// Event emitted when Owner AccountId Transferred
    #[ink(event)]
    #[metis(ownable)]
    pub struct OwnershipTransferred {
        /// previous owner account id
        #[ink(topic)]
        previous_owner: Option<AccountId>,
        /// new owner account id
        #[ink(topic)]
        new_owner: Option<AccountId>,
    }

    /// Event emitted when Pause
    #[ink(event)]
    #[metis(pausable)]
    pub struct Paused {
        /// paused caller
        #[ink(topic)]
        account: AccountId,
    }

    /// Event emitted when unPause
    #[ink(event)]
    #[metis(pausable)]
    pub struct Unpaused {
        /// unpaused caller
        #[ink(topic)]
        account: AccountId,
    }

    // impl
    impl Erc20Pausable {
        /// Creates a new ERC-20 contract with the specified initial supply.
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            let mut instance = Self {
                erc20: erc20::Data::new(),
                ownable: ownable::Data::new(),
                pausable: pausable::Data::new(),
            };

            erc20::Impl::init(
                &mut instance,
                String::from("MetisTestToken"),
                String::from("MET"),
                18_u8,
                initial_supply,
            );
            ownable::Impl::init(&mut instance);
            pausable::Impl::init(&mut instance);

            instance
        }

        // ERC20 messages
        #[ink(message)]
        pub fn name(&self) -> String {
            erc20::Impl::name(self)
        }

        #[ink(message)]
        pub fn symbol(&self) -> String {
            erc20::Impl::symbol(self)
        }

        #[ink(message)]
        pub fn decimals(&self) -> u8 {
            18_u8
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            erc20::Impl::total_supply(self)
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            erc20::Impl::balance_of(self, &owner)
        }

        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            erc20::Impl::allowance(self, &owner, &spender)
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            erc20::Impl::transfer(self, &to, value)
        }

        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            erc20::Impl::approve(self, &spender, value)
        }

        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {
            erc20::Impl::transfer_from(self, &from, &to, value)
        }

        // Owner messages
        #[ink(message)]
        pub fn get_ownership(&self) -> Option<AccountId> {
            *ownable::Impl::owner(self)
        }

        #[ink(message)]
        pub fn renounce_ownership(&mut self) {
            ownable::Impl::renounce_ownership(self)
        }

        #[ink(message)]
        pub fn transfer_ownership(&mut self, new_owner: AccountId) {
            ownable::Impl::transfer_ownership(self, &new_owner)
        }

        // Pausable messages
        #[ink(message)]
        pub fn paused(&self) -> bool {
            pausable::Impl::paused(self)
        }

        #[ink(message)]
        pub fn pause(&mut self) {
            ownable::Impl::ensure_caller_is_owner(self);
            pausable::Impl::_pause(self)
        }

        #[ink(message)]
        pub fn unpause(&mut self) {
            ownable::Impl::ensure_caller_is_owner(self);
            pausable::Impl::_unpause(self)
        }
    }

    /// Unit tests.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        use ink_env::{
            hash::{
                Blake2x256,
                CryptoHash,
                HashOutput,
            },
            Clear,
        };

        use erc20::Error;

        type Event = <Erc20Pausable as ::ink_lang::BaseEvent>::Type;

        use ink_lang as ink;

        // Test For Erc20Pausable
        #[ink::test]
        #[should_panic]
        fn cannot_transfer_when_paused() {
            // Constructor works.
            let mut erc20 = Erc20Pausable::new(100);

            erc20.pause();
            assert_eq!(erc20.paused(), true);

            transfer_works_by(&mut erc20, 2);
        }

        #[ink::test]
        #[should_panic]
        fn cannot_transfer_from_when_paused() {
            // Constructor works.
            let mut erc20 = Erc20Pausable::new(100);

            erc20.pause();
            assert_eq!(erc20.paused(), true);

            transfer_from_works_by(&mut erc20, 2);
        }

        #[ink::test]
        fn can_transfer_when_unpaused() {
            // Constructor works.
            let mut erc20 = Erc20Pausable::new(100);

            erc20.pause();
            erc20.unpause();

            assert_eq!(erc20.paused(), false);

            transfer_works_by(&mut erc20, 3);
        }

        #[ink::test]
        fn can_transfer_from_when_unpaused() {
            // Constructor works.
            let mut erc20 = Erc20Pausable::new(100);

            erc20.pause();
            erc20.unpause();

            assert_eq!(erc20.paused(), false);

            transfer_from_works_by(&mut erc20, 3);
        }

        fn assert_transfer_event(
            event: &ink_env::test::EmittedEvent,
            expected_from: Option<AccountId>,
            expected_to: Option<AccountId>,
            expected_value: Balance,
        ) {
            let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
                .expect("encountered invalid contract event data buffer");
            if let Event::Transfer(Transfer { from, to, value }) = decoded_event {
                assert_eq!(from, expected_from, "encountered invalid Transfer.from");
                assert_eq!(to, expected_to, "encountered invalid Transfer.to");
                assert_eq!(value, expected_value, "encountered invalid Trasfer.value");
            } else {
                panic!("encountered unexpected event kind: expected a Transfer event")
            }

            fn encoded_into_hash<T>(entity: &T) -> Hash
            where
                T: scale::Encode,
            {
                let mut result = Hash::clear();
                let len_result = result.as_ref().len();
                let encoded = entity.encode();
                let len_encoded = encoded.len();
                if len_encoded <= len_result {
                    result.as_mut()[..len_encoded].copy_from_slice(&encoded);
                    return result
                }
                let mut hash_output =
                    <<Blake2x256 as HashOutput>::Type as Default>::default();
                <Blake2x256 as CryptoHash>::hash(&encoded, &mut hash_output);
                let copy_len = core::cmp::min(hash_output.len(), len_result);
                result.as_mut()[0..copy_len].copy_from_slice(&hash_output[0..copy_len]);
                result
            }

            let expected_topics = vec![
                encoded_into_hash(&PrefixedValue {
                    value: b"Erc20Pausable::Transfer",
                    prefix: b"",
                }),
                encoded_into_hash(&PrefixedValue {
                    prefix: b"Erc20Pausable::Transfer::from",
                    value: &expected_from,
                }),
                encoded_into_hash(&PrefixedValue {
                    prefix: b"Erc20Pausable::Transfer::to",
                    value: &expected_to,
                }),
                encoded_into_hash(&PrefixedValue {
                    prefix: b"Erc20Pausable::Transfer::value",
                    value: &expected_value,
                }),
            ];
            for (n, (actual_topic, expected_topic)) in
                event.topics.iter().zip(expected_topics).enumerate()
            {
                let topic = actual_topic
                    .decode::<Hash>()
                    .expect("encountered invalid topic encoding");
                assert_eq!(topic, expected_topic, "encountered invalid topic at {}", n);
            }
        }

        /// The default constructor does its job.
        #[ink::test]
        fn new_works() {
            // Constructor works.
            let erc20 = Erc20Pausable::new(100);

            // Transfer event triggered during initial construction.
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(1, emitted_events.len());

            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );

            let name = erc20.name();
            assert_eq!(String::from("MetisTestToken"), name);

            let symbol = erc20.symbol();
            assert_eq!(String::from("MET"), symbol);
        }

        /// The total supply was applied.
        #[ink::test]
        fn total_supply_works() {
            // Constructor works.
            let erc20 = Erc20Pausable::new(100);
            // Transfer event triggered during initial construction.
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
            // Get the token total supply.
            assert_eq!(erc20.total_supply(), 100);
        }

        /// Get the actual balance of an account.
        #[ink::test]
        fn balance_of_works() {
            // Constructor works
            let erc20 = Erc20Pausable::new(100);
            // Transfer event triggered during initial construction
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            // Alice owns all the tokens on deployment
            assert_eq!(erc20.balance_of(accounts.alice), 100);
            // Bob does not owns tokens
            assert_eq!(erc20.balance_of(accounts.bob), 0);
        }

        fn transfer_works_by(erc20: &mut Erc20Pausable, emitted_events_len: usize) {
            // Transfer event triggered during initial construction.
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");

            assert_eq!(erc20.balance_of(accounts.bob), 0);
            // Alice transfers 10 tokens to Bob.
            assert_eq!(erc20.transfer(accounts.bob, 10), Ok(()));
            // Bob owns 10 tokens.
            assert_eq!(erc20.balance_of(accounts.bob), 10);

            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 1 + emitted_events_len);
            // Check first transfer event related to ERC-20 instantiation.
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
            // Check the second transfer event relating to the actual trasfer.
            assert_transfer_event(
                &emitted_events[emitted_events_len],
                Some(AccountId::from([0x01; 32])),
                Some(AccountId::from([0x02; 32])),
                10,
            );
        }

        #[ink::test]
        fn transfer_works() {
            // Constructor works.
            let mut erc20 = Erc20Pausable::new(100);
            transfer_works_by(&mut erc20, 1);
        }

        #[ink::test]
        fn invalid_transfer_should_fail() {
            // Constructor works.
            let mut erc20 = Erc20Pausable::new(100);
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");

            assert_eq!(erc20.balance_of(accounts.bob), 0);
            // Get contract address.
            let callee = ink_env::account_id::<ink_env::DefaultEnvironment>()
                .unwrap_or([0x0; 32].into());
            // Create call
            let mut data =
                ink_env::test::CallData::new(ink_env::call::Selector::new([0x00; 4])); // balance_of
            data.push_arg(&accounts.bob);
            // Push the new execution context to set Bob as caller
            ink_env::test::push_execution_context::<ink_env::DefaultEnvironment>(
                accounts.bob,
                callee,
                1000000,
                1000000,
                data,
            );

            // Bob fails to transfers 10 tokens to Eve.
            assert_eq!(
                erc20.transfer(accounts.eve, 10),
                Err(Error::InsufficientBalance)
            );
            // Alice owns all the tokens.
            assert_eq!(erc20.balance_of(accounts.alice), 100);
            assert_eq!(erc20.balance_of(accounts.bob), 0);
            assert_eq!(erc20.balance_of(accounts.eve), 0);

            // Transfer event triggered during initial construction.
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 1);
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
        }

        fn transfer_from_works_by(erc20: &mut Erc20Pausable, emitted_events_len: usize) {
            // Transfer event triggered during initial construction.
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");

            // Bob fails to transfer tokens owned by Alice.
            assert_eq!(
                erc20.transfer_from(accounts.alice, accounts.eve, 10),
                Err(Error::InsufficientAllowance)
            );
            // Alice approves Bob for token transfers on her behalf.
            assert_eq!(erc20.approve(accounts.bob, 10), Ok(()));

            // The approve event takes place.
            assert_eq!(
                ink_env::test::recorded_events().count(),
                1 + emitted_events_len
            );

            // Get contract address.
            let callee = ink_env::account_id::<ink_env::DefaultEnvironment>()
                .unwrap_or([0x0; 32].into());
            // Create call.
            let mut data =
                ink_env::test::CallData::new(ink_env::call::Selector::new([0x00; 4])); // balance_of
            data.push_arg(&accounts.bob);
            // Push the new execution context to set Bob as caller.
            ink_env::test::push_execution_context::<ink_env::DefaultEnvironment>(
                accounts.bob,
                callee,
                1000000,
                1000000,
                data,
            );

            // Bob transfers tokens from Alice to Eve.
            assert_eq!(
                erc20.transfer_from(accounts.alice, accounts.eve, 10),
                Ok(())
            );
            // Eve owns tokens.
            assert_eq!(erc20.balance_of(accounts.eve), 10);

            // Check all transfer events that happened during the previous calls:
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 3 + emitted_events_len);
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
            // The second event `emitted_events[1]` is an Approve event that we skip checking.
            assert_transfer_event(
                &emitted_events[1 + emitted_events_len],
                Some(AccountId::from([0x01; 32])),
                Some(AccountId::from([0x05; 32])),
                10,
            );
        }

        #[ink::test]
        fn transfer_from_works() {
            // Constructor works.
            let mut erc20 = Erc20Pausable::new(100);
            transfer_from_works_by(&mut erc20, 1);
        }

        #[ink::test]
        fn allowance_must_not_change_on_failed_transfer() {
            let mut erc20 = Erc20Pausable::new(100);
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");

            // Alice approves Bob for token transfers on her behalf.
            let alice_balance = erc20.balance_of(accounts.alice);
            let initial_allowance = alice_balance + 2;
            assert_eq!(erc20.approve(accounts.bob, initial_allowance), Ok(()));

            // Get contract address.
            let callee = ink_env::account_id::<ink_env::DefaultEnvironment>()
                .unwrap_or([0x0; 32].into());
            // Create call.
            let mut data =
                ink_env::test::CallData::new(ink_env::call::Selector::new([0x00; 4])); // balance_of
            data.push_arg(&accounts.bob);
            // Push the new execution context to set Bob as caller.
            ink_env::test::push_execution_context::<ink_env::DefaultEnvironment>(
                accounts.bob,
                callee,
                1000000,
                1000000,
                data,
            );

            // Bob tries to transfer tokens from Alice to Eve.
            let emitted_events_before =
                ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(
                erc20.transfer_from(accounts.alice, accounts.eve, alice_balance + 1),
                Err(Error::InsufficientBalance)
            );
            // Allowance must have stayed the same
            assert_eq!(
                erc20.allowance(accounts.alice, accounts.bob),
                initial_allowance
            );
            // No more events must have been emitted
            let emitted_events_after =
                ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events_before.len(), emitted_events_after.len());
        }
    }

    /// For calculating the event topic hash.
    struct PrefixedValue<'a, 'b, T> {
        pub prefix: &'a [u8],
        pub value: &'b T,
    }

    impl<X> scale::Encode for PrefixedValue<'_, '_, X>
    where
        X: scale::Encode,
    {
        #[inline]
        fn size_hint(&self) -> usize {
            self.prefix.size_hint() + self.value.size_hint()
        }

        #[inline]
        fn encode_to<T: scale::Output + ?Sized>(&self, dest: &mut T) {
            self.prefix.encode_to(dest);
            self.value.encode_to(dest);
        }
    }
}
