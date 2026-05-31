#![cfg(test)]

use crate::{time_lock::{TimeLockVault, TimeLockVaultClient}};
use soroban_sdk::{testutils::{Address as _, Ledger}, Address, Env};

#[test]
#[should_panic(expected = "tokens are still locked")]
fn test_early_withdrawal_fails() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TimeLockVault);
    let client = TimeLockVaultClient::new(&env, &contract_id);

    let beneficiary = Address::generate(&env);
    
    // Set ledger time to 1000, lock until 2000
    env.ledger().set_timestamp(1000);
    client.init(&beneficiary, &2000);

    // Mock token for test omitted for brevity
    // Attempt withdrawal at time 1500 (should fail)
    env.ledger().set_timestamp(1500);
    let mock_token = Address::generate(&env);
    client.withdraw(&mock_token);
}
