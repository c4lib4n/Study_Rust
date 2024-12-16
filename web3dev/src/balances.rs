use std::collections::BTreeMap;
pub struct Pallet {
    balances: BTreeMap<String, u128>,
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, caller: &String, amount: u128) {
        self.balances.insert(caller.clone(), amount);
    }

    pub fn balance(&self, who: &String) -> u128 {
        *self.balances.get(who).unwrap_or(&0)
    }

    pub fn transfer(
        &mut self,
        caller: String,
        to: String,
        amount: u128,
    ) -> Result<(), &'static str> {
        let caller_balance = self.balance(&caller);
        let to_balance = self.balance(&to);

        let new_caller_balance = caller_balance.checked_sub(amount).ok_or("Not enough balance")?;
        let new_to_balance = to_balance.checked_add(amount).ok_or("Overflow")?;

        self.balances.insert(caller, new_caller_balance);
        self.balances.insert(to, new_to_balance);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn init_balance() {
        let mut balances = Pallet::new();
        assert_eq!(balances.balance(&"Alice".to_string()), 0);
        balances.set_balance(&"Alice".to_string(), 100);
        assert_eq!(balances.balance(&"Alice".to_string()), 100);
        assert_eq!(balances.balance(&"Bob".to_string()), 0);
    }
}

#[test]
fn transfer_balance(){
    let mut balances = Pallet::new();

    assert_eq!(balances.transfer("Alice".to_string(), "Bob".to_string(), 10), Err("Not enough balance"));
    balances.set_balance(&"Alice".to_string(), 10);
    assert_eq!(balances.transfer("Alice".to_string(), "Bob".to_string(), 5), Ok(()));

    assert_eq!(balances.balance(&"Alice".to_string()), 5);
    assert_eq!(balances.balance(&"Bob".to_string()), 5);

    balances.set_balance(&"Bob".to_string(), u128::MAX);
    assert_eq!(balances.transfer("Alice".to_string(), "Bob".to_string(), 5), Err("Overflow"));


}