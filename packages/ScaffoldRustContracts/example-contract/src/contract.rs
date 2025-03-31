use soroban_sdk::{contract, contractimpl, contractclient, Env};

#[contract]
pub struct Counter;

#[contractimpl]
impl Counter {
    pub fn increment(env: Env, value: u32) -> u32 {
        let key = "counter";
        let mut current = env.storage().instance().get::<_, u32>(&key).unwrap_or(0);
        current += value;
        env.storage().instance().set(&key, &current);
        current
    }

    pub fn get(env: Env) -> u32 {
        let key = "counter";
        env.storage().instance().get::<_, u32>(&key).unwrap_or(0)
    }
}

#[contractclient(name = "CounterContract")]
pub trait CounterContractClient {
    fn increment(value: u32) -> u32;
    fn get() -> u32;
}