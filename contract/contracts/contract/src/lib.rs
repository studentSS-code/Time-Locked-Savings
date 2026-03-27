#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Map};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Balance(Address),
    UnlockTime(Address),
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn deposit(env: Env, user: Address, amount: i128, lock_period: u64) {
        user.require_auth();

        let current_balance: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Balance(user.clone()))
            .unwrap_or(0);

        let current_unlock: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::UnlockTime(user.clone()))
            .unwrap_or(0);

        let current_time = env.ledger().timestamp();
        let new_unlock = current_time + lock_period;

        // If current unlock time is in future, keep the longer lock
        let final_unlock = if current_unlock > current_time && current_unlock > new_unlock {
            current_unlock
        } else {
            new_unlock
        };

        let new_balance = current_balance + amount;

        env.storage()
            .persistent()
            .set(&DataKey::Balance(user.clone()), &new_balance);
        env.storage()
            .persistent()
            .set(&DataKey::UnlockTime(user.clone()), &final_unlock);
    }

    pub fn withdraw(env: Env, user: Address) {
        user.require_auth();

        let balance: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Balance(user.clone()))
            .unwrap_or(0);

        let unlock_time: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::UnlockTime(user.clone()))
            .unwrap_or(0);

        let current_time = env.ledger().timestamp();

        assert!(current_time >= unlock_time, "Funds are still locked");
        assert!(balance > 0, "No funds to withdraw");

        // Clear storage after withdrawal
        env.storage()
            .persistent()
            .remove(&DataKey::Balance(user.clone()));
        env.storage()
            .persistent()
            .remove(&DataKey::UnlockTime(user.clone()));
    }

    pub fn get_balance(env: Env, user: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Balance(user))
            .unwrap_or(0)
    }

    pub fn get_unlock_time(env: Env, user: Address) -> u64 {
        env.storage()
            .persistent()
            .get(&DataKey::UnlockTime(user))
            .unwrap_or(0)
    }
}

mod test;
