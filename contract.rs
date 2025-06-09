use soroban_sdk::{contractimpl, Address, Env, Symbol, contracttype, Vec};

pub struct BuilderIDE;

#[contracttype]
pub struct ContractTemplate {
    pub creator: Address,
    pub code: Vec<u8>,
    pub name: Symbol,
}

#[contractimpl]
impl BuilderIDE {
    fn templates<'a>(env: &'a Env) -> Vec<'a, ContractTemplate> {
        env.storage().instance().get::<Vec<ContractTemplate>>(Symbol::short("templates")).unwrap_or(Vec::new(&env))
    }

    pub fn save_template(env: Env, name: Symbol, code: Vec<u8>) {
        let creator = env.invoker();
        let mut templates = Self::templates(&env);
        templates.push_back(ContractTemplate { creator, code, name });
        env.storage().instance().set(Symbol::short("templates"), &templates);
    }

    pub fn get_template(env: Env, index: u32) -> ContractTemplate {
        let templates = Self::templates(&env);
        templates[index as usize].clone()
    }
}
