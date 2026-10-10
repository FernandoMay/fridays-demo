#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, String, Symbol, Vec};

#[contract]
pub struct Contract;

// This is a sample contract. Replace this placeholder with your own contract logic.
// A corresponding test example is available in `test.rs`.
//
// For comprehensive examples, visit <https://github.com/stellar/soroban-examples>.
// The repository includes use cases for the Stellar ecosystem, such as data storage on
// the blockchain, token swaps, liquidity pools, and more.
//
// Refer to the official documentation:
// <https://developers.stellar.org/docs/build/smart-contracts/overview>.
#[contractimpl]
impl Contract {
    pub fn hello(env: Env, to: String) -> Vec<String> {
        vec![&env, String::from_str(&env, "Hello"), to]
    }
}

// ---------------------------------------------------------------------------
// Counter — this was written on day one but left commented out. The 2026-10
// revival activates it and covers it with tests (see test.rs).
// ---------------------------------------------------------------------------

#[contract]
pub struct CounterContract;

const COUNT_KEY: Symbol = symbol_short!("count");

#[contractimpl]
impl CounterContract {
    /// Increments the stored counter by one and returns the new value.
    pub fn increment(env: Env) -> u32 {
        let mut count: u32 = env.storage().persistent().get(&COUNT_KEY).unwrap_or(0);
        count += 1;
        env.storage().persistent().set(&COUNT_KEY, &count);
        count
    }

    /// Returns the stored counter, or 0 if never incremented.
    pub fn get(env: Env) -> u32 {
        env.storage().persistent().get(&COUNT_KEY).unwrap_or(0)
    }
}

mod test;
