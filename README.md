# PidChat PSP22 Token Contract

This repository contains the smart contract implementation of the PidChat token using ink! 4.2.1.

## Overview

The PidChat token is a PSP22 compliant token (similar to ERC20) built on the Lunes Nightly  ecosystem using ink! smart contracts. It implements the base PSP22 functionality along with metadata extensions.

## Technical Details

- Built with ink! v4.2.1
- Implements PSP22 standard and PSP22Metadata extension
- Written in Rust with nightly toolchain

## Features

- Standard PSP22 token functionality (transfer, approve, etc)
- Metadata support (name, symbol, decimals)
- Event emission for transfers and approvals
- Transfer history
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

## Contributors

- Jorge Almeida <jorg.almeida@pidchat.com>

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Acknowledgements

- [ink!](https://github.com/paritytech/ink)
- [Lunes Nightly](https://github.com/lunes-io/lunes-nightly)


