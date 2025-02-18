#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![feature(min_specialization)]
#![warn(clippy::arithmetic_side_effects)]
use ink::env::Environment;
type DefaultAccountId = <ink::env::DefaultEnvironment as Environment>::AccountId;
type DefaultBalance = <ink::env::DefaultEnvironment as Environment>::Balance;

pub mod psp22 {
    use ink::prelude::vec::Vec;
    use crate::{DefaultAccountId, DefaultBalance,PSP22Error};

    #[ink::trait_definition]
    pub trait Psp22 {
        #[ink(message)]
        fn token_name(&self) -> Vec<u8>;

        #[ink(message)]
        fn token_symbol(&self) -> Vec<u8>;

        #[ink(message)]
        fn token_decimals(&self) -> u8;

        #[ink(message)]
        fn total_supply(&self) -> DefaultBalance;

        #[ink(message)]
        fn balance_of(&self, owner: DefaultAccountId) -> DefaultBalance;

        #[ink(message)]
        fn allowance(&self, owner: DefaultAccountId, spender: DefaultAccountId) -> DefaultBalance;

        #[ink(message)]
        fn transfer(&mut self, to: DefaultAccountId, value: DefaultBalance) -> Result<(), PSP22Error>;

        #[ink(message)]
        fn transfer_from(&mut self, from: DefaultAccountId, to: DefaultAccountId, value: DefaultBalance) -> Result<(),PSP22Error>;

        #[ink(message)]
        fn approve(&mut self, spender: DefaultAccountId, value: DefaultBalance) -> Result<(),PSP22Error>;

        #[ink(message)]
        fn increase_allowance(&mut self, spender: DefaultAccountId, value: DefaultBalance) -> Result<(),PSP22Error>;

        #[ink(message)]
        fn decrease_allowance(&mut self, spender: DefaultAccountId, value: DefaultBalance) -> Result<(),PSP22Error>;       

        #[ink(message)]
        fn burn(&mut self, from: DefaultAccountId, value: DefaultBalance) -> Result<(),PSP22Error>;

        #[ink(message)]
        fn history(&self, page: u32, limit: u32) -> Vec<(DefaultAccountId, DefaultAccountId, DefaultBalance, u64)>;
    }
}
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PSP22Error {
    TransferFailed,
    TransferFromFailed,
    ApproveFailed,
    IncreaseAllowanceFailed,
    DecreaseAllowanceFailed,
    BalanceNoAllocated,
    InsufficientBalance,
    InsufficientAllowance,
}
impl PSP22Error {
    pub fn from_error(error: PSP22Error) -> Self {
        match error {
            PSP22Error::TransferFailed => Self::TransferFailed,
            PSP22Error::TransferFromFailed => Self::TransferFromFailed,
            PSP22Error::ApproveFailed => Self::ApproveFailed,
            PSP22Error::IncreaseAllowanceFailed => Self::IncreaseAllowanceFailed,
            PSP22Error::DecreaseAllowanceFailed => Self::DecreaseAllowanceFailed,
            PSP22Error::BalanceNoAllocated => Self::BalanceNoAllocated,
            PSP22Error::InsufficientBalance => Self::InsufficientBalance,
            PSP22Error::InsufficientAllowance => Self::InsufficientAllowance,
        }
    }
}

#[ink::contract]
pub mod token {
    
    use super::{
        psp22::Psp22,
        DefaultAccountId,
        DefaultBalance,
        PSP22Error,
    };
    use ink_storage::Mapping;
    use ink::prelude::vec::Vec;
    use ink::prelude::string::String;
    use ink::prelude::string::ToString;
    #[ink(storage)]
    #[derive(Debug, Default)]
    pub struct PidChatPSP22 {
        name: Option<String>,
        symbol: Option<String>,
        decimals: u8,
        allowances: Mapping<(DefaultAccountId, DefaultAccountId), DefaultBalance>,
        balances: Mapping<DefaultAccountId, DefaultBalance>,
        total_supply: DefaultBalance,
        transfers: Mapping<DefaultAccountId, Vec<(DefaultAccountId, DefaultAccountId, DefaultBalance,u64)>>,
    }
    impl  PidChatPSP22 {
        #[ink(constructor)]
        pub fn new(
            total_supply: Balance,
            name: Option<String>,
            symbol: Option<String>,
            decimals: u8,
        ) -> Self {
            let mut instance = Self::default();
            instance.name = name.or_else(|| Some("PidChat".to_string()));
            instance.symbol = symbol.or_else(|| Some("PID".to_string()));
            instance.decimals = decimals;
            instance.total_supply = total_supply;
            instance.allowances = Mapping::new();
            instance.balances = Mapping::new();
            instance.balances.insert(&Self::env().caller(), &total_supply);
            instance.transfers = Mapping::new();
            instance
        }
         // Helper function to record transfers
         fn record_transfer(&mut self, from: DefaultAccountId, to: DefaultAccountId, value: DefaultBalance) {
            let timestamp = Self::env().block_timestamp();
            
            // Record transfer in sender's history
            let transfer_from = (from, to, value, timestamp);
            let mut transfers_from = self.transfers.get(&from).unwrap_or_default();
            transfers_from.push(transfer_from);
            self.transfers.insert(&from, &transfers_from);
            
            // Record transfer in recipient's history
            let transfer_to = (from, to, value, timestamp);
            let mut transfers_to = self.transfers.get(&to).unwrap_or_default();
            transfers_to.push(transfer_to);
            self.transfers.insert(&to, &transfers_to);
        }

        // Helper function to update balances
        fn update_balances(&mut self, from: DefaultAccountId, to: DefaultAccountId, value: DefaultBalance) -> Result<(), PSP22Error> {
            let from_balance = self.balances.get(&from).unwrap_or(0);
            if from_balance < value {
                return Err(PSP22Error::InsufficientBalance);
            }
            
            self.balances.insert(&from, &(from_balance - value));
            self.balances.insert(&to, &(self.balances.get(&to).unwrap_or(0) + value));
            Ok(())
        }
       
    }

    impl Psp22 for PidChatPSP22 {
        #[ink(message)]
        fn token_name(&self) -> Vec<u8> {
            self.name.clone().unwrap_or_default().into()
        }

        #[ink(message)]
        fn token_symbol(&self) -> Vec<u8> {
            self.symbol.clone().unwrap_or_default().into()
        }

        #[ink(message)]
        fn token_decimals(&self) -> u8 {
            self.decimals
        }
        #[ink(message)]
        fn total_supply(&self) -> DefaultBalance {
            self.total_supply.into()
        }

        #[ink(message)]
        fn balance_of(&self, owner: DefaultAccountId) -> DefaultBalance {
            self.balances.get(&owner).unwrap_or(0)
        }

        #[ink(message)]
        fn allowance(&self, owner: DefaultAccountId, spender: DefaultAccountId) -> DefaultBalance {
            self.allowances.get(&(owner, spender)).unwrap_or(0)
        }

        #[ink(message)]
        fn transfer(&mut self, to: DefaultAccountId, value: DefaultBalance) -> Result<(), PSP22Error> {
            let caller = Self::env().caller();
            // Update balances using helper function
            self.update_balances(caller, to, value)?;
            // Record transfer using helper function
            self.record_transfer(caller, to, value);
            Ok(())
        }

        #[ink(message)]
        fn transfer_from(&mut self, from: DefaultAccountId, to: DefaultAccountId, value: DefaultBalance) -> Result<(),PSP22Error> {
            let caller = Self::env().caller();
            
            // Check allowance
            let allowance = self.allowances.get(&(from, caller)).unwrap_or(0);
            if allowance < value {
                return Err(PSP22Error::InsufficientAllowance);
            }
            
            // Update balances using helper function
            self.update_balances(from, to, value)?;
            // Remove allowance after successful transfer
            self.allowances.remove(&(from, caller));
            // Record transfer using helper function
            self.record_transfer(from, to, value);
            Ok(())
             
        }

        #[ink(message)]
        fn approve(&mut self, spender: DefaultAccountId, value: DefaultBalance) -> Result<(),PSP22Error> {
            self.allowances.insert(&(Self::env().caller(), spender), &value);
            Ok(())
        }

        #[ink(message)]
        fn increase_allowance(&mut self, spender: DefaultAccountId, value: DefaultBalance) -> Result<(),PSP22Error> {
            let caller = Self::env().caller();          
            //update the allowance
            self.allowances.insert(&(caller, spender), &(self.allowances.get(&(caller, spender)).unwrap_or(0) + value));
            Ok(())
        }

        #[ink(message)]
        fn decrease_allowance(&mut self, spender: DefaultAccountId, value: DefaultBalance) -> Result<(),PSP22Error> {
            let caller = Self::env().caller();
            let allowance = self.allowances.get(&(caller, spender)).unwrap_or(0);
            if allowance < value {
                return Err(PSP22Error::InsufficientAllowance);
            }
            //update the allowance
            self.allowances.insert(&(caller, spender), &(allowance - value));
            Ok(())
        }

        #[ink(message)]
        fn burn(&mut self, from: DefaultAccountId, value: DefaultBalance) -> Result<(),PSP22Error> {
            let caller = Self::env().caller();
            let balance = self.balances.get(&caller).unwrap_or(0);
            if balance < value {
                return Err(PSP22Error::InsufficientBalance);
            }
            //update the balance
            self.balances.insert(&caller, &(balance - value));
            self.total_supply = self.total_supply - value;
            Ok(())
        }
        #[ink(message)]
        fn history(&self, page: u32, limit: u32) -> Vec<(DefaultAccountId, DefaultAccountId, DefaultBalance, u64)> {
            let caller = self.env().caller();
            let transfers = self.transfers.get(&caller).unwrap_or_default();
            
            // Validate pagination parameters
            if page == 0 || limit == 0 {
                return Vec::new();
            }

            // Calculate pagination indices with overflow protection
            let start = ((page - 1) * limit) as usize;
            if start >= transfers.len() {
                return Vec::new();
            }

            let end = (start + limit as usize).min(transfers.len());
            
            // Return the requested slice of history
            transfers[start..end].to_vec()
        }
        
    }
}