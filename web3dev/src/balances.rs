use std::collections::BTreeMap;
use num::traits::{CheckedAdd, CheckedSub, Zero};


#[derive(Debug)]
pub struct Pallet<AccountId, Balance> {
    balances: BTreeMap<AccountId, Balance>,
}

impl <AccountId, Balance> Pallet <AccountId, Balance>

where
AccountId: Ord + Clone,
Balance: CheckedAdd + CheckedSub + Zero + Copy,

{
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, caller: &AccountId, amount: Balance) {
        self.balances.insert(caller.clone(), amount);
    }

    pub fn balance(&self, who: &AccountId) -> Balance {
        *self.balances.get(who).unwrap_or(&Balance::zero())
    }

    pub fn transfer(
        &mut self,
        caller: AccountId,
        to: AccountId,
        amount: Balance,
    ) -> Result<(), &'static str> {
        let caller_balance = self.balance(&caller);
        let to_balance = self.balance(&to);

        let new_caller_balance = caller_balance.checked_sub(&amount).ok_or("Not enough balance")?;
        let new_to_balance = to_balance.checked_add(&amount).ok_or("Overflow")?;

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
        let mut balances = Pallet::<String, u128>::new();
        assert_eq!(balances.balance(&"Alice".to_string()), 0);
        balances.set_balance(&"Alice".to_string(), 100);
        assert_eq!(balances.balance(&"Alice".to_string()), 100);
        assert_eq!(balances.balance(&"Bob".to_string()), 0);
    }
}

#[test]
fn transfer_balance(){
    let mut balances = Pallet::<String, u128>::new();

    assert_eq!(balances.transfer("Alice".to_string(), "Bob".to_string(), 10), Err("Not enough balance"));
    balances.set_balance(&"Alice".to_string(), 10);
    assert_eq!(balances.transfer("Alice".to_string(), "Bob".to_string(), 5), Ok(()));

    assert_eq!(balances.balance(&"Alice".to_string()), 5);
    assert_eq!(balances.balance(&"Bob".to_string()), 5);

    balances.set_balance(&"Bob".to_string(), u128::MAX);
    assert_eq!(balances.transfer("Alice".to_string(), "Bob".to_string(), 5), Err("Overflow"));


}