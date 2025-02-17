# PidChat PSP22 Token Contract

This repository contains the smart contract implementation of the PidChat token using ink! 4.2.1 and OpenBrush 4.0.0-beta.

## Overview

The PidChat token is a PSP22 compliant token (similar to ERC20) built on the Lunes Nightly  ecosystem using ink! smart contracts. It implements the base PSP22 functionality along with metadata extensions.

## Technical Details

- Built with ink! v4.2.1
- Uses OpenBrush v4.0.0-beta for PSP22 implementation
- Implements PSP22 standard and PSP22Metadata extension
- Written in Rust with nightly toolchain

## Features

- Standard PSP22 token functionality (transfer, approve, etc)
- Metadata support (name, symbol, decimals)
- Event emission for transfers and approvals

## Building

1. Install Rust and Cargo:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install --force --locked cargo-contract
```

2. Build the contract:
```bash
   cd contracts/psp22 && cargo contract build --release
```

3. Run tests:
```bash
   cd contracts/psp22 && cargo contract test
```

4. Generate the Wasm binary:
```bash 
    cd contracts/psp22 && cargo contract build-wasm
```


## Builds Artifacts
```bash
 npm run compile:release
```
## Node Lunes Nightly for testing
 1 . clone the repository
```bash
   git clone https://github.com/lunes-io/lunes-nightly.git
```
 2 . install dependencies
```bash
   docker compose up -d
```

# Deploy Contract
 1 . deploy the contract
    https://ui.use.ink/

 2. Select Local Node

 3. Select the contract

 4. Deploy

 5. Copy the contract address

## Contract Interface

### Constructor

```rust 
    #[ink(constructor)]
    pub fn new(
        total_supply: Balance,
        name: Option<String>,
        symbol: Option<String>,
        decimals: u8,
    ) -> Self {
        let mut instance = Self::default();
        psp22::Internal::_mint_to(&mut instance, Self::env().caller(), total_supply).expect("Error minting tokens"); 
        instance.metadata.name.set(&name);
        instance.metadata.symbol.set(&symbol);
        instance.metadata.decimals.set(&decimals);            
        instance
    }   
```

### Transfer Event

```rust     
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }
```

### Approval Event

```rust
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }
```

### Storage

```rust
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct PidChatPSP22 {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        metadata: metadata::Data,
    }   
```

### Transfer Message

```rust 
    #[ink(message)]
    pub fn transfer(
        &mut self,
        to: AccountId,
        value: Balance,
    ) -> Result<(), PSP22Error> {   
        psp22::Internal::_transfer(self, from, to, value)
    }
```

### Approval Message

```rust 
    #[ink(message)]
    pub fn approve(
        &mut self,
        spender: AccountId,
        value: Balance,
    ) -> Result<(), PSP22Error> {   
        psp22::Internal::_approve(self, spender, value)
    }
```

### TransferFrom Message    

```rust 
    #[ink(message)]
    pub fn transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,  
        value: Balance,
    ) -> Result<(), PSP22Error> {   
        psp22::Internal::_transfer_from(self, from, to, value)
    }
```

### Approve Message     

```rust     
    #[ink(message)]
    pub fn approve(
        &mut self,
        spender: AccountId,
        value: Balance,
    ) -> Result<(), PSP22Error> {   
        psp22::Internal::_approve(self, spender, value)
    }
```

### Burn Message

```rust
    #[ink(message)]
    pub fn burn(
        &mut self,
        value: Balance,
    ) -> Result<(), PSP22Error> {   
        psp22::Internal::_burn(self, value)
    }
```

## Contributors

- Jorge Almeida <jorg.almeida@pidchat.com>

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Acknowledgements

- [OpenBrush](https://github.com/Brushfam/openbrush-contracts)
- [ink!](https://github.com/paritytech/ink)
- [Lunes Nightly](https://github.com/lunes-io/lunes-nightly)


