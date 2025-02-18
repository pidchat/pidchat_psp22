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
    use ink::codegen::{
        EmitEvent,
        Env,
    };
    use ink_storage::Mapping;
    use ink::prelude::vec::Vec;
    use ink::prelude::string::String;
    use ink::prelude::string::ToString;
    #[ink(storage)]
    #[derive(Default)]
    pub struct PidChatPSP22 {
        name: Option<String>,
        symbol: Option<String>,
        decimals: u8,
        allowances: Mapping<(DefaultAccountId, DefaultAccountId), DefaultBalance>,
        balances: Mapping<DefaultAccountId, DefaultBalance>,
        total_supply: DefaultBalance,
        transfers: Mapping<DefaultAccountId, Vec<(DefaultAccountId, DefaultAccountId, DefaultBalance,u64)>>,
    }

    // Define the Transfer event
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    // Define the Approval event
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
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
            // Remove the oldest transfer if the history is full
            if transfers_from.len() >= 100 {
                // Remove first transfer or oldest transfer
                transfers_from.remove(0);
            }
            transfers_from.push(transfer_from);
            self.transfers.insert(&from, &transfers_from);
            
            // Record transfer in recipient's history
            let transfer_to = (from, to, value, timestamp);
            let mut transfers_to = self.transfers.get(&to).unwrap_or_default();
            // Remove the oldest transfer if the history is full
            if transfers_to.len() >= 100 {
                // Remove first transfer or oldest transfer
                transfers_to.remove(0);
            }
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
        // Helper function to emit transfer events
        fn _emit_transfer_event(
            &self,
            from: Option<AccountId>,
            to: Option<AccountId>,
            amount: Balance,
        ) {
            self.env().emit_event(Transfer {
                from,
                to,
                value: amount,
            });
        }
        // Helper function to emit approval events
        fn _emit_approval_event(&self, owner: AccountId, spender: AccountId, amount: Balance) {
            self.env().emit_event(Approval {
                owner,
                spender,
                value: amount,
            });
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
            // Emit transfer event using helper function
            self._emit_transfer_event(Some(caller), Some(to), value);
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
            // Emit transfer event using helper function
            self._emit_transfer_event(Some(from), Some(to), value);
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
    #[cfg(test)]
    mod tests {
        use super::*;    

        use ink::env::test::{default_accounts, set_caller}; 
        use ink::env::DefaultEnvironment;
        use crate::Environment;
        type Balance = <DefaultEnvironment as Environment>::Balance;

        // Helper function to setup test environment
        fn setup() -> PidChatPSP22 {        
            let total_supply: Balance = 1_000_000;
            PidChatPSP22::new(
                total_supply,
                Some("TestToken".to_string()),
                Some("TST".to_string()),
                18,
            )
        }

        // Test basic token information
        #[ink::test]
        fn test_token_info() {
            let contract = setup();
            
            assert_eq!(contract.token_name(), "TestToken".as_bytes().to_vec());
            assert_eq!(contract.token_symbol(), "TST".as_bytes().to_vec());
            assert_eq!(contract.token_decimals(), 18);
            assert_eq!(contract.total_supply(), 1_000_000);
        }

        // Test initial balance
        #[ink::test]
        fn test_initial_balance() {
            let contract = setup();
            let accounts = default_accounts::<DefaultEnvironment>();
            
            assert_eq!(contract.balance_of(accounts.alice), 1_000_000);
            assert_eq!(contract.balance_of(accounts.bob), 0);
        }

        // Test transfer
        #[ink::test]
        fn test_transfer() {
            let mut contract = setup();
            let accounts = default_accounts::<DefaultEnvironment>();
            
            // Transfer 100 tokens from Alice to Bob
            assert!(contract.transfer(accounts.bob, 100).is_ok());
            
            // Check balances after transfer
            assert_eq!(contract.balance_of(accounts.alice), 1_000_000 - 100);
            assert_eq!(contract.balance_of(accounts.bob), 100);
        }

        // Test transfer with insufficient balance
        #[ink::test]
        fn test_transfer_insufficient_balance() {
            let mut contract = setup();
            let accounts = default_accounts::<DefaultEnvironment>();
            
            // Try to transfer more than available balance
            let result = contract.transfer(accounts.bob, 2_000_000);
            assert_eq!(result, Err(PSP22Error::InsufficientBalance));
        }

        // Test approve and allowance
        #[ink::test]
        fn test_approve_and_allowance() {
            let mut contract = setup();
            let accounts = default_accounts::<DefaultEnvironment>();
            
            // Approve Bob to spend 500 tokens
            assert!(contract.approve(accounts.bob, 500).is_ok());
            
            // Check allowance
            assert_eq!(contract.allowance(accounts.alice, accounts.bob), 500);
        }

        // Test transfer_from
        #[ink::test]
        fn test_transfer_from() {
            let mut contract = setup();
            let accounts = default_accounts::<DefaultEnvironment>();
            
            // Alice approves Bob to spend 500 tokens
            assert!(contract.approve(accounts.bob, 500).is_ok());
            
            // Bob transfers 300 tokens from Alice to Charlie
            set_caller::<DefaultEnvironment>(accounts.bob);
            assert!(contract.transfer_from(accounts.alice, accounts.charlie, 300).is_ok());
            
            // Check balances
            assert_eq!(contract.balance_of(accounts.alice), 1_000_000 - 300);
            assert_eq!(contract.balance_of(accounts.charlie), 300);
            assert_eq!(contract.allowance(accounts.alice, accounts.bob), 0);
        }

        // Test increase/decrease allowance
        #[ink::test]
        fn test_allowance_modifications() {
            let mut contract = setup();
            let accounts = default_accounts::<DefaultEnvironment>();
            
            // Initial approve
            assert!(contract.approve(accounts.bob, 500).is_ok());
            
            // Increase allowance
            assert!(contract.increase_allowance(accounts.bob, 200).is_ok());
            assert_eq!(contract.allowance(accounts.alice, accounts.bob), 700);
            
            // Decrease allowance
            assert!(contract.decrease_allowance(accounts.bob, 300).is_ok());
            assert_eq!(contract.allowance(accounts.alice, accounts.bob), 400);
        }

        // Test burn
        #[ink::test]
        fn test_burn() {
            let mut contract = setup();
            let accounts = default_accounts::<DefaultEnvironment>();
            
            // Burn 100 tokens
            assert!(contract.burn(accounts.alice, 100).is_ok());
            
            // Check balance and total supply
            assert_eq!(contract.balance_of(accounts.alice), 1_000_000 - 100);
            assert_eq!(contract.total_supply(), 1_000_000 - 100);
        }

        // Test transfer history
        #[ink::test]
        fn test_transfer_history() {
            let mut contract = setup();
            let accounts = default_accounts::<DefaultEnvironment>();
            
            // Make some transfers
            assert!(contract.transfer(accounts.bob, 100).is_ok());
            assert!(contract.transfer(accounts.charlie, 200).is_ok());
            
            // Check history for Alice
            let history = contract.history(1, 10);
            assert_eq!(history.len(), 2);
            
            // Verify first transfer details
            let (from, to, value, _) = history[0];
            assert_eq!(from, accounts.alice);
            assert_eq!(to, accounts.bob);
            assert_eq!(value, 100);
        }

        // Test history pagination
        #[ink::test]
        fn test_history_pagination() {
            let mut contract = setup();
            let accounts = default_accounts::<DefaultEnvironment>();
            
            // Make multiple transfers
            for i in 0..5 {
                assert!(contract.transfer(accounts.bob, 100 + i).is_ok());
            }
            
            // Test first page
            let page1 = contract.history(1, 2);
            assert_eq!(page1.len(), 2);
            
            // Test second page
            let page2 = contract.history(2, 2);
            assert_eq!(page2.len(), 2);
            
            // Test last page
            let page3 = contract.history(3, 2);
            assert_eq!(page3.len(), 1);
            
            // Test invalid page
            let invalid_page = contract.history(10, 2);
            assert_eq!(invalid_page.len(), 0);
        }

        // Test maximum history size
        #[ink::test]
        fn test_max_history_size() {
            let mut contract = setup();
            let accounts = default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.alice);
            // Make more transfers than MAX_HISTORY_SIZE
            for i in 0..200 {
                assert!(contract.transfer(accounts.bob, 100 + i).is_ok());
            }
            
            // Check that history is limited to MAX_HISTORY_SIZE
            let history = contract.history(1, 100);
            assert_eq!(history.len(), 100); // Assuming MAX_HISTORY_SIZE is 100
        }
    }
}
