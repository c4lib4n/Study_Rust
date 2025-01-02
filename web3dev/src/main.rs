mod balances;
mod system;

mod types {
    pub type AccountId = String;
    pub type Balance = u128;
    pub type Nonce = u32;
    pub type BlockNumber = u32;
}

#[derive(Debug)]
pub struct Runtime {
    balances: balances::Pallet<Self>,
    system: system::Pallet<Self>,
}

impl balances::Config for Runtime {
    type Balance = types::Balance;

}
impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;

}


impl Runtime{
    pub fn new() -> Self{
        Runtime{
            balances: balances::Pallet::new(),
            system: system::Pallet::new(),
        }

    }
}
use std::collections::BTreeMap;
use crate::system::Config;
use crate::types::{AccountId, Balance};

fn main() {

    let mut runtime = Runtime::new();

    runtime.balances.set_balance(&"Alice".to_string(), 100);
    runtime.system.inc_block_number();
    assert_eq!(runtime.system.block_number(), 1);

    runtime.system.inc_nonce(&"Alice".to_string());
    let _res = runtime.balances.transfer("Alice".to_string(), "Bob".to_string(),30).map_err(|e| println!("err {:?}", e));

    runtime.system.inc_nonce(&"Alice".to_string());
    let _res = runtime.balances.transfer("Alice".to_string(), "Charlie".to_string(), 20);


    println!("{:#?}", runtime);









}
