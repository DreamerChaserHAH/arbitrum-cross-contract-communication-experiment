# arbitrum-cross-contract-communication-experiment

> Experimenting with layered function calling using Arbiturm Stylus (Rust)

## 1.0 Contracts
There are two branches:
- <a href="https://github.com/DreamerChaserHAH/arbitrum-cross-contract-communication-experiment/tree/communicator-smart-contract">Communicator Smart Contract</a>
- <a href="https://github.com/DreamerChaserHAH/arbitrum-cross-contract-communication-experiment/tree/receiver-smart-contract">Receiver Smart Contract</a>

### Communicator Smart Contract

```
Supposedly, this smart contract would be the one that will be visible to the public
```

### Receiver Smart Contract

```
only allow execution of functions from specific smart contracts (which is set by the owner)
```

## 2.0 How to deploy
Here are the general steps:
1. Deploy Receiver Smart Contract (Remember the deployment address)
2. call the init() method
3. change the RECEIVER_SMART_CONTRACT constant variable inside the communicator smart contract rust file source code.
4. Deploy the communicator smart contract (remember the deployment address)
5. call the setSmartContract function and set the contract address

### 2.1 Commands

Note that, the private key supplied is the dev node private key that is made available on Arbitrum Quickstart Page

#### 2.1.1 For deployment
- Make sure you have launched the docker engine before you deploy
```
cargo stylus deploy \   
  --endpoint='http://localhost:8547' \
  --private-key="0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659"
```

#### 2.1.1.1 Receiver Smart Contract Commands

##### 2.1.1.1.1 Calling the init method
```
cast send --rpc-url "http://localhost:8547" --private-key 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659 [deployed-address-of-receiver-contract] "init()"
```

##### 2.1.1.1.2 View the number

```
cast call --rpc-url "http://localhost:8547" --private-key 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659 [deployed-address-of-receiver-contract] "viewNumber()(uint256)"
```

##### 2.1.1.1.3 Update the Communicator Smart Contract

```
cast send --rpc-url "http://localhost:8547" --private-key 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659 [your-received-smart-contract] "setSmartContract(address)" [your-deployed-communicator-smart-contract]
```

#### 2.1.1.2 Communicator Smart Contract
##### Executing

```
cast send --rpc-url "http://localhost:8547" --private-key 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659 0x408da76e87511429485c32e4ad647dd14823fdc4 "execute()"    
```

##### Viewing
```
cast call --rpc-url "http://localhost:8547" --private-key 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659 0x408da76e87511429485c32e4ad647dd14823fdc4 "view()"    
```