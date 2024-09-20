use alloy::{primitives::hex, sol, sol_types::SolCall};
use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use log;

use serde::Deserialize;
#[derive(Deserialize)]
struct Row {
    transaction_hash: String,
    input: String,
}

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

async fn read_csv<P: AsRef<Path>>(filename: P) -> Result<(), Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut rows = csv::Reader::from_reader(file);

    for row in rows.deserialize() {
        let row: Row = row.unwrap();

        // Convert strings from CSV to hex.
        let calldata = hex::decode(&row.input);
        let decoded = match calldata {
            Ok(data) => execTransactionCall::abi_decode(&data, false),
            Err(e) => {
                log::error!("Error decoding calldata: {:?}", e);
                return Err(Box::new(e));
            }
        };

        match decoded {
            Ok(decoded) => {
                let operation = decoded.operation.as_u8().clone();
                // We only need to output where it's a delegate-call or operation=1
                if operation == 1 {
                    println!("{:},{:},{:},{:},{:}",
                        &row.transaction_hash,
                        &row.input,
                        &decoded.operation.as_u8(),
                        &decoded.data,
                        &decoded.to
                    );
                }
            }
            Err(e) => {
                log::error!("Error decoding transaction: {:?} {:?}", &row.transaction_hash, e);
            }
        }
    }

    Ok(())
}

async fn process_directory(directory: &str) -> Result<(), Box<dyn Error>> {
    let paths = std::fs::read_dir(directory)?;
    
    // Iterate over files in the directory
    for entry in paths {
        let path = entry?.path();
        if path.is_file() {
            // Convert path to a string and pass it to read_csv
            if let Some(filename) = path.to_str() {
                read_csv(filename).await?;
            }
        }
    }

    Ok(())
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Sets the input directory containing CSV files
    #[arg(required = true)]
    directory: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    // Set up CLI arguments using Clap
    let args = Args::parse();

    // Process each file in the directory
    process_directory(&args.directory).await?;

    Ok(())
}
