use std::env;
use alloy::{
    eips::BlockId, network::EthereumWallet, 
    primitives::{b256, Address}, 
    providers::{Provider, ProviderBuilder}, rpc::types::Filter, 
    signers::local::PrivateKeySigner, sol, sol_types::SolCall,
    node_bindings::Anvil,
};
use clap::Parser;
use url::Url;
use log;

//extern crate clap;
//extern crate csv;
//extern crate env_logger;

sol!(
    #[allow(missing_docs)]
    #[derive(Debug)]
    enum Operation {
        Call, 
        DelegateCall,
    }

    #[allow(missing_docs)]
    #[derive(Debug)]
    function execTransaction(
        address to,
        uint256 value,
        bytes calldata data,
        Operation operation,
        uint256 safeTxGas,
        uint256 baseGas,
        uint256 gasPrice,
        address gasToken,
        address payable refundReceiver,
        bytes memory signatures
    ) public payable virtual returns (bool success);
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    GNOSIS,
    "src/gnosis.json"
);

// This is the decode contract in bytes. You need to build using forge build then navigate to the build `out` directory, 
// find the SafeDedoder.sol json and copy out the deployed_bytecode and copy here.
sol! {
    #[allow(missing_docs)]
    // Needs to bytecode not deployed_bytecode.
    #[sol(rpc, bytecode="0x608060405234801561001057600080fd5b50610881806100206000396000f3fe608060405234801561001057600080fd5b506004361061004c5760003560e01c806338eada1c14610051578063a39fac1214610066578063ba4dbcc414610084578063edf26d9b14610097575b600080fd5b61006461005f36600461061a565b6100c2565b005b61006e610109565b60405161007b919061064a565b60405180910390f35b61006e61009236600461073a565b61016b565b6100aa6100a53660046107af565b610591565b6040516001600160a01b03909116815260200161007b565b6100cb816105bb565b610106576000805460018101825590805260008051602061080c8339815191520180546001600160a01b0319166001600160a01b0383161790555b50565b6060600080548060200260200160405190810160405280929190818152602001828054801561016157602002820191906000526020600020905b81546001600160a01b03168152600190910190602001808311610143575b5050505050905090565b60606000808060008060005b8781101561057a5760418181028a0160208101516040820151919092015160ff169550909350915060008490036102495788820160200180519395508593906101bf856105bb565b6101fa576000805460018101825590805260008051602061080c8339815191520180546001600160a01b0319166001600160a01b0389161790555b866001600160a01b031660008051602061082c83398151915260405161023a9060208082526006908201526507620697320360d41b604082015260600190565b60405180910390a2505061052b565b8360ff166001036102eb578260001c9450610263856105bb565b61029e576000805460018101825590805260008051602061080c8339815191520180546001600160a01b0319166001600160a01b0387161790555b846001600160a01b031660008051602061082c8339815191526040516102de9060208082526006908201526576206973203160d01b604082015260600190565b60405180910390a261052b565b601e8460ff161115610432576040517f19457468657265756d205369676e6564204d6573736167653a0a3332000000006020820152603c81018c9052600190605c016040516020818303038152906040528051906020012060048661035091906107c8565b6040805160008152602081018083529390935260ff90911690820152606081018590526080810184905260a0016020604051602081039080840390855afa15801561039f573d6000803e3d6000fd5b5050506020604051035194506103b4856105bb565b6103ef576000805460018101825590805260008051602061080c8339815191520180546001600160a01b0319166001600160a01b0387161790555b846001600160a01b031660008051602061082c8339815191526040516102de90602080825260099082015268076206973203e2033360bc1b604082015260600190565b6040805160008152602081018083528d905260ff861691810191909152606081018490526080810183905260019060a0016020604051602081039080840390855afa158015610485573d6000803e3d6000fd5b50505060206040510351945061049a856105bb565b6104d5576000805460018101825590805260008051602061080c8339815191520180546001600160a01b0319166001600160a01b0387161790555b846001600160a01b031660008051602061082c833981519152604051610522906020808252601390820152727620697320736f6d657468696e6720656c736560681b604082015260600190565b60405180910390a25b849550610537856105bb565b610572576000805460018101825590805260008051602061080c8339815191520180546001600160a01b0319166001600160a01b0387161790555b600101610177565b610582610109565b9b9a5050505050505050505050565b600081815481106105a157600080fd5b6000918252602090912001546001600160a01b0316905081565b6000805b60005481101561061157826001600160a01b0316600082815481106105e6576105e66107f5565b6000918252602090912001546001600160a01b0316036106095750600192915050565b6001016105bf565b50600092915050565b60006020828403121561062c57600080fd5b81356001600160a01b038116811461064357600080fd5b9392505050565b6020808252825182820181905260009190848201906040850190845b8181101561068b5783516001600160a01b031683529284019291840191600101610666565b50909695505050505050565b634e487b7160e01b600052604160045260246000fd5b600082601f8301126106be57600080fd5b813567ffffffffffffffff808211156106d9576106d9610697565b604051601f8301601f19908116603f0116810190828211818310171561070157610701610697565b8160405283815286602085880101111561071a57600080fd5b836020870160208301376000602085830101528094505050505092915050565b6000806000806080858703121561075057600080fd5b84359350602085013567ffffffffffffffff8082111561076f57600080fd5b61077b888389016106ad565b9450604087013591508082111561079157600080fd5b5061079e878288016106ad565b949793965093946060013593505050565b6000602082840312156107c157600080fd5b5035919050565b60ff82811682821603908111156107ef57634e487b7160e01b600052601160045260246000fd5b92915050565b634e487b7160e01b600052603260045260246000fdfe290decd9548b62a8d60345a988386fc84ba6bc95484008f6362f93160ef3e56360aa51fba1c22b339436d9ebffcff47836158524b19cd99c10d5ac0fd9136b11a264697066735822122006f9991b9c3a281208a144b00491f074447f3ecbafcb070c61ce84361e60009c64736f6c63430008180033")]
    contract SafeDecoder {
        address[] public addresses;
    
        function returnSigners(
            bytes32 dataHash,
            bytes memory data,
            bytes memory signatures,
            uint256 requiredSignatures
        ) public returns (address[] memory){}
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "0x27fD43BABfbe83a81d14665b1a6fB8030A60C9b4")]
    safe_wallet_address: String,

    #[arg(long, default_value_t = 16132578)]
    start_block: u64,

    #[arg(long, default_value_t = 20331566)]
    end_block: u64,

    #[arg(long, default_value_t = 10000)]
    chunk_size: u64,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = Args::parse();

    let rpc = env::var("ETH_RPC_URL").map_err(|e| format!("Failed to get ETH_RPC_URL: {}", e))?;
    let rpc_url = Url::parse(&rpc).map_err(|e| format!("Failed to parse RPC URL: {}", e))?;
    let provider = ProviderBuilder::new().on_http(rpc_url);
    
    // ExecutionSuccess (bytes32 txHash, uint256 payment)
    let execution_event = b256!("442e715f626346e8c54381002da614f62bee8d27386535b2521ec8540898556e");

    // Parser args or use defaults
    let safe_wallet_address = Address::parse_checksummed(args.safe_wallet_address, None).expect("Invalid address provided");
    let start_block = args.start_block;
    let end_block = args.end_block;
    let chunk_size = args.chunk_size; 

    // Should only need to create anvil once and deploy the contract then call it per log.
    let anvil = Anvil::new().fork(&rpc).try_spawn().unwrap();
    log::info!("Anvil running at `{}`", anvil.endpoint());

    let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
    let wallet = EthereumWallet::from(signer);

    let local_url = anvil.endpoint().parse().unwrap();

    // Not sure we need Anvil yet. Try and query the contract at a particular block
    let local_provider =
        ProviderBuilder::new().with_recommended_fillers().wallet(wallet).on_http(local_url);

    // Deploy the decoder contract.
    let decoder_contract = SafeDecoder::deploy(&local_provider).await;
    let safe_address = decoder_contract.unwrap().address().clone();

    let safe_decoder = SafeDecoder::new(safe_address, &local_provider);

    println!("block_number,tx_hash,tx_hash,data_hash,nonce,threshold,calldata,signers");

    for from_block in (start_block..=end_block).step_by(chunk_size as usize) {
        // Calculate to_block for this chunk, ensuring it doesn't exceed the end_block
        let to_block = std::cmp::min(from_block + chunk_size - 1, end_block);

        // Process each block range (from_block to to_block)
        //println!("Processing from block: {} to block: {}", from_block, to_block);

        // 10,000 block limit on log queries. This query has the `address` and `event_signature` combined.
        let filter = Filter::new().address(safe_wallet_address).event_signature(execution_event).from_block(from_block).to_block(to_block);

        // Get all logs from the latest block that match the filter.
        let logs = provider.get_logs(&filter).await.unwrap();
        // Get all the logs for the event from block to block.
        // We'll build a Vec of the transactions and then process and decode each.
        for log in logs {
            //println!("Logs: {:}", &log.transaction_hash.unwrap());
            let tx = provider.get_transaction_by_hash(log.transaction_hash.unwrap()).await.unwrap();
            let tx_bn = tx.clone();
            let block_number_before = tx_bn.clone().unwrap().block_number.unwrap() - 1;
            //let block_before = provider.get_block_by_number(block_number_before.into(), false).await.unwrap();

            let decoded = execTransactionCall::abi_decode(&tx.unwrap().input, false);

            match decoded {
                Ok(decoded) => {
                    let operation = decoded.operation.as_u8().clone();
                    let call_data = decoded.data.clone();

                    // if operation != 1 {
                    //    continue 
                    // }
                    
                    // Query contract and get the current nonce and threshold.
                    let contract = GNOSIS::new(safe_wallet_address, &provider); 
                    // I am assuming there aren't multiple transactions in a single block for the same safe wallet.
                    // It is a reasonable assumption for high value treasury wallets but means some nonces may change 
                    // a number of times in a single block. 
                    let nonce  = match contract
                        .nonce()
                        .block(BlockId::from(block_number_before))
                        .call()
                        .await 
                    {
                        Ok(nonce) => nonce,
                        Err(e) => {
                            log::error!("Error");
                            return Err(e.into())
                        }
                    };
                    // Similar to above. I am assuming that threshold is not changed in the same block
                    // as the transaction matching the event.
                    let threshhold = match contract
                        .getThreshold()
                        .block(BlockId::from(block_number_before))
                        .call()
                        .await 
                    {
                        Ok(t) => t,
                        Err(e) => {
                            log::error!("Error");
                            return Err(e.into())
                        }
                    };
                    // It's there if you need it.
                    // let _owners = match contract.
                    //     getOwners()
                    //     .block(BlockId::from(block_number_before))
                    //     .call()
                    //     .await 
                    // {
                    //     Ok(owners) => owners,
                    //     Err(e) => {
                    //         log::error!("Error");
                    //         return Err(e.into())
                    //     }
                    // };

                    let tx_hash_result = contract
                            .getTransactionHash(
                                decoded.to, 
                                decoded.value, 
                                decoded.data.clone(), 
                                operation, 
                                decoded.safeTxGas, 
                                decoded.baseGas, 
                                decoded.gasPrice, 
                                decoded.gasToken, 
                                decoded.refundReceiver, 
                                nonce._0
                            )
                            .block(BlockId::from(block_number_before))
                            .call()
                            .await;

                    let tx_hash = match tx_hash_result {
                        Ok(result) => result._0,
                        Err(e) => {
                            log::error!("Failed to get transaction hash: {}", e);
                            return Err(e.into());
                        }
                    };
                    // Calling encodeTransaction will give us the safe tx dataHash
                    let data_hash_result = contract
                        .encodeTransactionData(
                            decoded.to,
                            decoded.value,
                            decoded.data,
                            operation,
                            decoded.safeTxGas,
                            decoded.baseGas,
                            decoded.gasPrice,
                            decoded.gasToken,
                            decoded.refundReceiver,
                            nonce._0
                        )
                        .block(BlockId::from(block_number_before))
                        .call()
                        .await;

                    let data_hash = match data_hash_result {
                        Ok(result) => result._0,
                        Err(e) => {
                            log::error!("Failed to get data hash: {}", e);
                            return Err(e.into());
                        }
                    };

                    let signers = safe_decoder
                        .returnSigners(
                            tx_hash.clone(),
                            data_hash.clone(),
                            decoded.signatures,
                            threshhold._0
                        )
                        .call()
                        .await;

                    match signers {
                        Ok(signers) => {
                            let signer_list = signers._0.iter()
                                .map(|signer| signer.to_string())
                                .collect::<Vec<String>>()
                                .join(", ");

                            // Handle unwraps before println
                            let block_number = match tx_bn.clone().and_then(|bn| bn.block_number) {
                                Some(number) => number.to_string(),
                                None => "Unknown".to_string(),
                            };

                            let transaction_hash = log.transaction_hash
                                .map(|hash| hash.to_string())
                                .unwrap_or_else(|| "Unknown".to_string());

                            println!(
                                "{},{},{},{},{},{},{},\"[{}]\"",
                                block_number,
                                transaction_hash,
                                tx_hash,
                                data_hash,
                                nonce._0,
                                threshhold._0,
                                call_data,
                                signer_list
                            );
                        },
                        Err(e) => {
                            log::error!("Couldn't recover signers");
                            return Err(e.into())
                        }
                    }
                }
                Err(e) => {
                    log::info!("Error decoding input: {e:?}");
                }
            }
        }
    }
    Ok(())
}
