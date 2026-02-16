# Neurotoken

Neurotoken is a Rust crate providing an in-memory token ledger with accounts, minting, burning, and transfers, intended as a foundation for neuro-inspired or DeFi-style experiments.

## Features

- Simple `Ledger` with account creation, balances, and supplies.
- `TokenMeta` and `TokenSupply` for metadata and supply tracking.
- No `parking_lot::RwLock`; single-threaded, explicit ownership.
- Ready to embed into services or CLIs.

## Quick example

```rust
use neurotoken::{init_logging, Ledger, TokenMeta};

fn main() {
    init_logging();
    let mut ledger = Ledger::new();

    let alice = ledger.create_account("alice").unwrap();
    let bob = ledger.create_account("bob").unwrap();

    let meta = TokenMeta::new("NEURO", 6, "Neuro Token");
    ledger.mint(&alice, 1_000_000, meta.clone()).unwrap();

    ledger
        .transfer(neurotoken::TransferRequest::new(
            alice.clone(),
            bob.clone(),
            250_000,
            meta.symbol.clone(),
        ))
        .unwrap();

    println!("alice: {}", ledger.balance_of(&alice, &meta.symbol).unwrap());
    println!("bob:   {}", ledger.balance_of(&bob, &meta.symbol).unwrap());
}
