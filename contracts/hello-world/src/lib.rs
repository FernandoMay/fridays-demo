#![no_std]
use soroban_sdk::{contract, contractimpl, vec, Env, String, Vec};

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

mod test;

// #![no_std]
// use soroban_sdk::{contract, contractimpl, Env, Symbol};

// #[contract]
// pub struct CounterContract;

// #[contractimpl]
// impl CounterContract {
//     pub fn increment(env: Env) -> u32 {
//         let key = Symbol::short("count");
//         let mut count: u32 = env.storage().persistent().get(&key).unwrap_or(0);
//         count += 1;
//         env.storage().persistent().set(&key, &count);
//         count
//     }

//     pub fn get(env: Env) -> u32 {
//         let key = Symbol::short("count");
//         env.storage().persistent().get(&key).unwrap_or(0)
//     }
// }
