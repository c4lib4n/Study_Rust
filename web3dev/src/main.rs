mod balances;
mod system;

pub struct Runtime{
    balances: balances::Pallet,
    system: system::Pallet,
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


fn main() {

    println!("Hello, world!");

}
