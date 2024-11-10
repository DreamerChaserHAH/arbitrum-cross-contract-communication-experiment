//!
//! Stylus Receiver Smart Contract
//! 
//! Description: this smart contract only receives and process messages from another smart contract

#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use alloy_primitives::U256;
use stylus_sdk::{msg, prelude::*};


sol_storage! {
    #[entrypoint]
    pub struct ReceiverSmartContract {
        bool initialized;
        address owner;
        address executable_smart_contract;
        uint256 number;
    }
}

#[public]
impl ReceiverSmartContract {
    fn init(&mut self) -> Result<(), Vec<u8>>{
        let initialized = self.initialized.get();
        if initialized {
            return Err("Already initialized".into());
        }

        self.initialized.set(true);
        self.owner.set(msg::sender());
        self.number.set(U256::from(0));

        Ok(())
    }

    fn set_smart_contract(&mut self, new_communicator_smart_contract: alloy_primitives::Address) -> Result<(), Vec<u8>>{
        let initialized = self.initialized.get();
        if !initialized {
            return Err("Not initialized".into());
        }
        
        let owner = self.owner.get();
        if owner != msg::sender() {
            return Err("Only the owner can set the executable smart contract".into());
        }

        self.executable_smart_contract.set(new_communicator_smart_contract);
        Ok(())
    }

    fn view_number(&self) -> U256{
        self.number.get()
    }

    fn execute(&mut self) -> Result<(), Vec<u8>>{
        let initialized = self.initialized.get();
        if !initialized {
            return Err("Not initialized".into());
        }

        let executable_smart_contract = self.executable_smart_contract.get();
        if executable_smart_contract != msg::sender() {
            return Err("Only the executable smart contract can execute the execute method".into());
        }

        let number = self.number.get();
        self.number.set(number + U256::from(1));

        Ok(())
    }
}
