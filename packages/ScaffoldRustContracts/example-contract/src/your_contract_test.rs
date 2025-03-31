use soroban_sdk::{
    Env, Address, symbol_short, IntoVal,
    testutils::{Address as _, MockAuth, MockAuthInvoke},
};

use crate::{ExampleContract, ExampleContractClient};

#[test]
fn test_initialize_and_set_get() {
    let env = Env::default();
    let contract_id = env.register(ExampleContract, ());
    let client = ExampleContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);

    env.mock_auths(&[MockAuth {
        address: &admin,
        invoke: &MockAuthInvoke {
            contract: &contract_id,
            fn_name: "initialize".into(),
            args: (admin.clone(),).into_val(&env),
            sub_invokes: &[],
        },
    }]);

    client.initialize(&admin);
    assert_eq!(client.get_admin(), admin);

    env.mock_auths(&[MockAuth {
        address: &admin,
        invoke: &MockAuthInvoke {
            contract: &contract_id,
            fn_name: "set".into(),
            args: (
                symbol_short!("foo"),
                symbol_short!("bar")
            ).into_val(&env),
            sub_invokes: &[],
        },
    }]);

    client.set(&symbol_short!("foo"), &symbol_short!("bar"));

    let value = client.get(&symbol_short!("foo"));
    assert_eq!(value, Some(symbol_short!("bar")));
}

#[test]
#[should_panic(expected = "Already initialized")]
fn test_double_initialize_panics() {
    let env = Env::default();
    let contract_id = env.register(ExampleContract, ());
    let client = ExampleContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);

    env.mock_auths(&[MockAuth {
        address: &admin,
        invoke: &MockAuthInvoke {
            contract: &contract_id,
            fn_name: "initialize".into(),
            args: (admin.clone(),).into_val(&env),
            sub_invokes: &[],
        },
    }]);

    client.initialize(&admin);

    env.mock_auths(&[MockAuth {
        address: &admin,
        invoke: &MockAuthInvoke {
            contract: &contract_id,
            fn_name: "initialize".into(),
            args: (admin.clone(),).into_val(&env),
            sub_invokes: &[],
        },
    }]);

    client.initialize(&admin); // 🔥 esto debe hacer panic
}
