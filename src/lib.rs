#![no_std]
use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct VaultContract;

#[contractimpl]
impl VaultContract {
    pub fn deposit() {}
}
