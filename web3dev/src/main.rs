mod balances;
use std::collections::BTreeMap;
use crate::balances::Pallet;

fn main() {
    let mut pallet = Pallet::new();
    pallet.set_balance(&"Matheus".to_string(), 10);

    let balance = pallet.balance(&"Matheus".to_string());
    println!("Balance: {}", balance);



}
