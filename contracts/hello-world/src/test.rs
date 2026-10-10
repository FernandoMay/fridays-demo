#![cfg(test)]

use super::*;
use soroban_sdk::{vec, Env, String};

#[test]
fn test_hello() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let words = client.hello(&String::from_str(&env, "Dev"));
    assert_eq!(
        words,
        vec![
            &env,
            String::from_str(&env, "Hello"),
            String::from_str(&env, "Dev"),
        ]
    );
}

#[test]
fn test_hello_with_other_name() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let words = client.hello(&String::from_str(&env, "Fernando"));
    assert_eq!(words.len(), 2);
    assert_eq!(words.get(1).unwrap(), String::from_str(&env, "Fernando"));
}

#[test]
fn test_counter_starts_at_zero() {
    let env = Env::default();
    let contract_id = env.register(CounterContract, ());
    let client = CounterContractClient::new(&env, &contract_id);

    assert_eq!(client.get(), 0);
}

#[test]
fn test_counter_increments_and_persists() {
    let env = Env::default();
    let contract_id = env.register(CounterContract, ());
    let client = CounterContractClient::new(&env, &contract_id);

    assert_eq!(client.increment(), 1);
    assert_eq!(client.increment(), 2);
    assert_eq!(client.increment(), 3);
    assert_eq!(client.get(), 3);
}
