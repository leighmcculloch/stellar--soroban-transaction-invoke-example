# Generating a transaction for a contract

## Download

```
git clone ...
cd ..
```

## Contract

```rust
#![no_std]
use soroban_sdk::{contractimpl, Env, Symbol};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn put(e: Env, key: Symbol, val: u32) {
        e.contract_data().set((e.invoker(), key), val)
    }
}
```

## Usage

### Generate a transaction

```
cargo run
```
