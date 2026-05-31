#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Address};

#[contract]
pub struct VaultContract;

#[contractimpl]
impl VaultContract {
    pub fn deposit(env: Env, from: Address, amount: i128) {}
    pub fn withdraw(env: Env, to: Address, amount: i128) {}
    pub fn execute_timelock(env: Env) {}
}
