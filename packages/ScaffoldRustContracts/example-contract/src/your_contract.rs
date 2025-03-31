use soroban_sdk::{contract, contractimpl, Env, Symbol, Address, symbol_short};

#[contract]
pub struct ExampleContract;

#[contractimpl]
impl ExampleContract {
    pub fn initialize(env: Env, admin: Address) {
        let key = symbol_short!("ADMIN");
        if env.storage().instance().has(&key) {
            panic!("Already initialized");
        }
        admin.require_auth();
        env.storage().instance().set(&key, &admin);
    }

    pub fn set(env: Env, key: Symbol, value: Symbol) {
        let admin_key = symbol_short!("ADMIN");
        let admin: Address = env.storage().instance().get(&admin_key).unwrap();
        admin.require_auth();
        env.storage().instance().set(&key, &value);
    }

    pub fn get(env: Env, key: Symbol) -> Option<Symbol> {
        env.storage().instance().get(&key)
    }

    pub fn get_admin(env: Env) -> Address {
        let key = symbol_short!("ADMIN");
        env.storage().instance().get(&key).unwrap_or_else(|| {
            panic!("Admin not set. Call initialize() first.")
        })
    }
}
