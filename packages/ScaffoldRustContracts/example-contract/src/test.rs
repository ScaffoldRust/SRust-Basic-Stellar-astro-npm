use soroban_sdk::Env;
use crate::{Counter, CounterContract};

#[test]
fn test_counter() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Counter {});
    let client = CounterContract::new(&env, &contract_id);

    assert_eq!(client.get(), 0);
    assert_eq!(client.increment(&5), 5);
    assert_eq!(client.increment(&3), 8);
}
