#![cfg_attr(not(feature = "std"), no_std)]

pub use self::erc20::Erc20Stub;
use ink_lang as ink;

#[ink::contract]
mod erc20 {
    use ink_prelude::string::String;

    /// The ERC-20 error types.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Returned if not enough balance to fulfill a request is available.
        InsufficientBalance,
        /// Returned if not enough allowance to fulfill a request is available.
        InsufficientAllowance,
    }

    /// The ERC-20 result type.
    pub type Result<T> = core::result::Result<T, Error>;

    #[ink(storage)]
    pub struct Erc20Stub {}

    impl Erc20Stub {
        /// Creates a new ERC-20 contract with the specified initial supply.
        #[ink(constructor)]
        pub fn new(
            _initial_supply: Balance,
            _name: String,
            _symbol: String,
            _decimals: Option<u8>,
        ) -> Self {
            unimplemented!()
        }

        /// Returns the token name.
        #[ink(message)]
        pub fn token_name(&self) -> String {
            unimplemented!()
        }

        /// Returns the token symbol.
        #[ink(message)]
        pub fn token_symbol(&self) -> String {
            unimplemented!()
        }

        /// Returns the token decimals.
        #[ink(message)]
        pub fn token_decimals(&self) -> u8 {
            unimplemented!()
        }

        /// Returns the total token supply.
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            unimplemented!()
        }

        /// Returns the account balance for the specified `owner`.
        #[ink(message)]
        pub fn balance_of(&self, _owner: AccountId) -> Balance {
            unimplemented!()
        }

        /// Transfers `value` amount of tokens from the caller's account to account `to`.
        #[ink(message)]
        pub fn transfer(&mut self, _to: AccountId, _value: Balance) -> Result<()> {
            unimplemented!()
        }

        /// Returns the amount which `spender` is still allowed to withdraw from `owner`.
        #[ink(message)]
        pub fn allowance(&self, _owner: AccountId, _spender: AccountId) -> Balance {
            unimplemented!()
        }

        /// Transfers `value` tokens on the behalf of `from` to the account `to`.
        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            _from: AccountId,
            _to: AccountId,
            _value: Balance,
        ) -> Result<()> {
            unimplemented!()
        }

        /// Allows `spender` to withdraw from the caller's account multiple times, up to
        /// the `value` amount.
        #[ink(message)]
        pub fn approve(&mut self, _spender: AccountId, _value: Balance) -> Result<()> {
            unimplemented!()
        }
    }
}
