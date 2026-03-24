#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol, symbol_short};

#[contracttype]
#[derive(Clone)]
pub struct Deposit {
    pub amount: i128,
    pub unlock_time: u64,
}

const DEPOSIT_KEY: Symbol = symbol_short!("DEP");

#[contract]
pub struct TimeLockedSavings;

#[contractimpl]
impl TimeLockedSavings {
    // Deposit funds with a lock time
    pub fn deposit(env: Env, user: Address, amount: i128, unlock_time: u64) {
        user.require_auth();

        let current_time = env.ledger().timestamp();

        if unlock_time <= current_time {
            panic!("Unlock time must be in the future");
        }

        let deposit = Deposit { amount, unlock_time };

        env.storage().persistent().set(&(DEPOSIT_KEY, user.clone()), &deposit);
    }

    // Withdraw funds after unlock time
    pub fn withdraw(env: Env, user: Address) -> i128 {
        user.require_auth();

        let key = (DEPOSIT_KEY, user.clone());

        let deposit: Deposit = env
            .storage()
            .persistent()
            .get(&key)
            .expect("No deposit found");

        let current_time = env.ledger().timestamp();

        if current_time < deposit.unlock_time {
            panic!("Funds are still locked");
        }

        env.storage().persistent().remove(&key);

        deposit.amount
    }

    // View deposit details
    pub fn get_deposit(env: Env, user: Address) -> Deposit {
        env.storage()
            .persistent()
            .get(&(DEPOSIT_KEY, user))
            .expect("No deposit found")
    }
}