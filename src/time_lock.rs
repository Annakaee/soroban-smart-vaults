#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env};

#[contracttype]
pub enum TimeLockDataKey {
    Beneficiary,
    ReleaseTime,
}

#[contract]
pub struct TimeLockVault;

#[contractimpl]
impl TimeLockVault {
    pub fn init(env: Env, beneficiary: Address, release_time: u64) {
        if env.storage().instance().has(&TimeLockDataKey::Beneficiary) {
            panic!("already initialized");
        }
        env.storage().instance().set(&TimeLockDataKey::Beneficiary, &beneficiary);
        env.storage().instance().set(&TimeLockDataKey::ReleaseTime, &release_time);
    }

    pub fn deposit(env: Env, token: Address, from: Address, amount: i128) {
        from.require_auth();
        let client = token::Client::new(&env, &token);
        client.transfer(&from, &env.current_contract_address(), &amount);
    }

    pub fn withdraw(env: Env, token: Address) {
        let beneficiary: Address = env.storage().instance().get(&TimeLockDataKey::Beneficiary).unwrap();
        beneficiary.require_auth();

        let release_time: u64 = env.storage().instance().get(&TimeLockDataKey::ReleaseTime).unwrap();
        let current_time = env.ledger().timestamp();
        
        if current_time < release_time {
            panic!("tokens are still locked");
        }

        let client = token::Client::new(&env, &token);
        let balance = client.balance(&env.current_contract_address());
        client.transfer(&env.current_contract_address(), &beneficiary, &balance);
    }
}
