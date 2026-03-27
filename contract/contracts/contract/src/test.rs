#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_deposit_and_withdraw() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let user = Address::generate(&env);

    // Deposit 1000 for 10 seconds
    client.deposit(&user, &1000i128, &10u64);

    // Check balance
    let balance = client.get_balance(&user);
    assert_eq!(balance, 1000i128);

    // Check unlock time
    let unlock_time = client.get_unlock_time(&user);
    let current = env.ledger().timestamp();
    assert!(unlock_time > current);
}

#[test]
fn test_cannot_withdraw_before_unlock() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let user = Address::generate(&env);

    // Deposit with 100 second lock
    client.deposit(&user, &500i128, &100u64);

    // Try to withdraw before unlock - should fail
    let result = client.try_withdraw(&user);
    assert!(result.is_err());
}

#[test]
fn test_withdraw_after_unlock() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let user = Address::generate(&env);

    // Fast-forward ledger to simulate time passing
    env.ledger().set_sequence(1000);

    // Deposit with very short lock (0 seconds = immediate unlock on same ledger)
    client.deposit(&user, &500i128, &0u64);

    // Withdraw should succeed now
    let result = client.try_withdraw(&user);
    assert!(result.is_ok());

    // Balance should be 0 after withdrawal
    let balance = client.get_balance(&user);
    assert_eq!(balance, 0i128);
}

#[test]
fn test_multiple_deposits() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let user = Address::generate(&env);

    // Multiple deposits
    client.deposit(&user, &100i128, &10u64);
    client.deposit(&user, &200i128, &20u64);
    client.deposit(&user, &300i128, &5u64);

    // Total balance
    let balance = client.get_balance(&user);
    assert_eq!(balance, 600i128);
}
