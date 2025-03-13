use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
    type Balance: CheckedAdd + CheckedSub + Zero + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    balances: BTreeMap<T::AccountId, T::Balance>,
}

#[macros::call]
impl<T: Config> Pallet<T> {
    pub fn transfer(
        &mut self,
        caller: T::AccountId,
        to: T::AccountId,
        amount: T::Balance,
    ) -> Result<(), &'static str> {
        let caller_balance = self.balance(&caller);
        let to_balance = self.balance(&to);

        let new_caller_balance = caller_balance
            .checked_sub(&amount)
            .ok_or("Not enough balance")?;
        let new_to_balance = to_balance.checked_add(&amount).ok_or("Overflow")?;

        self.balances.insert(caller, new_caller_balance);
        self.balances.insert(to, new_to_balance);

        Ok(())
    }
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, caller: &T::AccountId, amount: T::Balance) {
        self.balances.insert(caller.clone(), amount);
    }

    pub fn balance(&self, who: &T::AccountId) -> T::Balance {
        *self.balances.get(who).unwrap_or(&T::Balance::zero())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestConfig;

    impl super::Config for TestConfig {
        type Balance = u128;
    }

    impl crate::system::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn init_balance() {
        let mut balances = Pallet::<TestConfig>::new();
        assert_eq!(balances.balance(&"Alice".to_string()), 0);
        balances.set_balance(&"Alice".to_string(), 100);
        assert_eq!(balances.balance(&"Alice".to_string()), 100);
        assert_eq!(balances.balance(&"Bob".to_string()), 0);
    }

    #[test]
    fn transfer_balance() {
        let mut balances = Pallet::<TestConfig>::new();

        assert_eq!(
            balances.transfer("Alice".to_string(), "Bob".to_string(), 10),
            Err("Not enough balance")
        );
        balances.set_balance(&"Alice".to_string(), 10);
        assert_eq!(
            balances.transfer("Alice".to_string(), "Bob".to_string(), 5),
            Ok(())
        );

        assert_eq!(balances.balance(&"Alice".to_string()), 5);
        assert_eq!(balances.balance(&"Bob".to_string()), 5);

        balances.set_balance(&"Bob".to_string(), u128::MAX);
        assert_eq!(
            balances.transfer("Alice".to_string(), "Bob".to_string(), 5),
            Err("Overflow")
        );
    }
}
