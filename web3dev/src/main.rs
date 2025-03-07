mod balances;
mod support;
mod system;

mod types {
    use crate::support;
    pub type AccountId = String;
    pub type Balance = u128;
    pub type Nonce = u32;
    pub type BlockNumber = u32;
    pub type Extrinsic = support::Extrinsic<AccountId, crate::RuntimeCall>;
    pub type Header = support::Header<BlockNumber>;
    pub type Block = support::Block<Header, Extrinsic>;
}

pub enum RuntimeCall {
    Balances(balances::Call<Runtime>),
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

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            balances: balances::Pallet::new(),
            system: system::Pallet::new(),
        }
    }
    fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
        self.system.inc_block_number();
        if (self.system.block_number() != block.header.block_number) {
            return Err("Block number dismatch");
        }

        for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
            self.system.inc_nonce(&caller);
            let _ = self.dispatch(caller, call).map_err(|e| {
                eprintln!(
                    "Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
                    block.header.block_number, i, e
                )
            });
        }
        Ok(())
    }
}
impl crate::support::Dispatch for Runtime {
    type Caller = <Runtime as system::Config>::AccountId;
    type Call = RuntimeCall;
    // Dispatch a call on behalf of a caller. Increments the caller's nonce.
    //
    // Dispatch allows us to identify which underlying module call we want to execute.
    // Note that we extract the `caller` from the extrinsic, and use that information
    // to determine who we are executing the call on behalf of.
    fn dispatch(
        &mut self,
        caller: Self::Caller,
        runtime_call: Self::Call,
    ) -> support::DispatchResult {
        match runtime_call {
            RuntimeCall::Balances (call) => {
                self.balances.dispatch(caller, call)?;
            }
        }
        Ok(())
    }
}

use crate::support::Dispatch;
use crate::system::Config;
use crate::types::{AccountId, Balance};
use std::collections::BTreeMap;

fn main() {
    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();
    let mut runtime = Runtime::new();

    runtime.balances.set_balance(&alice, 100);

    let block_1 = types::Block{
        header: support::Header {block_number: 1},
        extrinsics: vec![
            support::Extrinsic{
                caller: alice.clone(),
                call: RuntimeCall::Balances(balances::Call::Transfer  {to: bob, amount: 30}),
            },
            support::Extrinsic{
                caller: alice,
                call: RuntimeCall::Balances (balances::Call::Transfer {to: charlie, amount: 20}),
            },
        ],
    };
    runtime.execute_block(block_1).expect("Wrong block execution");
    println!("{:#?}", runtime);
}
