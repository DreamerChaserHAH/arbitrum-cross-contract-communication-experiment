//!
//! Stylus Hello World
//!
//! This contract will be used to trigger the execute function from the receiver smart contract, ideally any wallet will be able to call it

// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use alloy_primitives::{address, Address, U256};
use stylus_sdk::prelude::*;

sol_interface!{
    interface IReceiverSmartContract {
        function init() external;
    
        function setSmartContract(address new_communicator_smart_contract) external;
    
        function viewNumber() external view returns (uint256);
    
        function execute() external;
    }
}

const RECEIVER_SMART_CONTRACT_ADDRESS: Address = address!("75e0e92a79880bd81a69f72983d03c75e2b33dc8");

#[storage]
#[entrypoint]
pub struct COMMUNICATOR_SMART_CONTRACT{
}

#[public]
impl COMMUNICATOR_SMART_CONTRACT{
    fn execute(&mut self) -> Result<(), Vec<u8>> {
        let receiver_smart_contract = IReceiverSmartContract::new(RECEIVER_SMART_CONTRACT_ADDRESS);
        receiver_smart_contract.execute(self);
        Ok(())
    }

    fn view(&self) -> Result<U256, Vec<u8>> {
        let receiver_smart_contract = IReceiverSmartContract::new(RECEIVER_SMART_CONTRACT_ADDRESS);
        let view_result = receiver_smart_contract.view_number(self);
        if view_result.is_err(){
            return Err(b"Failed to view the number".to_vec());
        }

        Ok(view_result.unwrap())
    }
}

