use jni::JNIEnv;

// These objects are what you should use as arguments to your native
// function. They carry extra lifetime information to prevent them escaping
// this context and getting used after being GC'd.
use jni::objects::{JClass, JString};

// This is just a pointer. We'll be returning it from our function. We
// can't return one of the objects with lifetime information because the
// lifetime checker won't let us.
use jni::sys::jstring;

use alloy::primitives::b256;
use alloy::primitives::{utils, Address};
use eyre::Result;

use helios::{client::ClientBuilder, config::networks::Network, prelude::*, types::BlockTag};
use std::{env, fs, io, path::Path, path::PathBuf, str::FromStr, thread, time::Duration};
use tokio::runtime::Runtime;

#[no_mangle]
pub extern "C" fn Java_com_example_foldtest_RustLib_getBalance(
    env: JNIEnv,
    _class: JClass,
) -> jstring {
    let rt = Runtime::new().unwrap();
    let temp_dir = env::temp_dir().join("helios");

    if let Err(e) = create_directory_if_not_exists(&temp_dir) {
        eprintln!(
            "[Helios-Android-JNI]: Failed to create helios directory: {}",
            e
        );
    } else {
        println!(
            "[Helios-Android-JNI]: Helios directory created or already exists at: {:?}",
            temp_dir
        );
    }

    // Use async mode to get the balance
    let result: Result<String, String> = rt.block_on(async {
        println!(
            "[Helios-Android-JNI]: Building client with data directory: {:?}",
            temp_dir
        );
        match create_client(temp_dir) {
            Ok(client) => {
                println!("[Helios-Android-JNI]: Client created successfully.");

                let res = match get_balance(client).await {
                    Ok(data) => {
                        println!("[Helios-Android-JNI]: Get balance successfully.");
                        data
                    }
                    Err(err_msg) => {
                        println!("[Helios-Android-JNI]: get balance falied, {}", err_msg);
                        String::from("[Helios-Android-JNI]: Failed to get balance with error")
                    }
                };

                Ok(res.to_string())
            }

            Err(err_msg) => {
                eprintln!("[Helios-Android-JNI]: Client creation error: {}", err_msg);
                Ok("[Helios-Android-JNI]: HSY Client created Failed.".to_string())
            }
        }
    });
    match result {
        Ok(balance) => env
            .new_string(balance)
            .expect("[Helios-Android-JNI]: Couldn't create Java string!")
            .into_raw(),
        Err(err_msg) => env
            .new_string(err_msg)
            .expect("[Helios-Android-JNI]: Couldn't create Java string!")
            .into_raw(),
    }
}

fn create_directory_if_not_exists(path: &Path) -> io::Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

fn create_client(temp_dir: PathBuf) -> Result<Client<FileDB>, String> {
    let untrusted_rpc_url = "https://eth-mainnet.g.alchemy.com/v2/yhqq51-m59s1dG5iJTq3cxDpxMwq39e-";

    let client = ClientBuilder::new()
        .network(Network::MAINNET)
        .consensus_rpc("https://www.lightclientdata.org")
        .execution_rpc(untrusted_rpc_url)
        .load_external_fallback()
        .data_dir(temp_dir)
        .checkpoint(b256!(
            "51c25b7f30627132fc636cd6b634ef4accf0a6c114c1adf8c808c122efdab276"
        ))
        .build()
        .map_err(|e| format!("Client build error: {}", e))?;

    Ok(client)
}

async fn get_balance(mut client: Client<FileDB>) -> Result<String, String> {
    println!("[Helios-Android-JNI]: Get balance start");

    client
        .start()
        .await
        .map_err(|e| format!("[Helios-Android-JNI]: Client start error: {}", e))?;

    println!("[Helios-Android-JNI][Siyuan han Magic]: Sleep for a while zzzzzzzzzzzzz");

    // Magic number
    let mut countdown = 15;
    while countdown > 0 {
        println!("Remaining time: {} seconds", countdown);
        thread::sleep(Duration::from_secs(1)); // 睡眠1秒
        countdown -= 1;
    }

    println!("[Helios-Android-JNI][Siyuan han Magic]: WWWWWWWWWake up!!!");

    let addr = Address::from_str("0x00000000219ab540356cBB839Cbe05303d7705Fa")
        .map_err(|e| format!("[Helios-Android-JNI]: Address parse error: {}", e))?;

    let block = BlockTag::Latest;

    let balance = client
        .get_balance(addr, block)
        .await
        .map_err(|e| format!("[Helios-Android-JNI]: Get balance error: {}", e))?;

    Ok(utils::format_ether(balance))
}
// The following code was used for step testing
// Will be deprecated after the stable library released.
// #[no_mangle]
// pub extern "C" fn Java_com_example_foldtest_RustLib_testClientStepByStepTest(
//     env: JNIEnv,
//     _class: JClass,
// ) -> jstring {
//     let mut builder = ClientBuilder::new();

//     // Set the network to mainnet
//     builder = builder.network(networks::Network::MAINNET);

//     // // Set the consensus rpc url
//     builder = builder.consensus_rpc("https://www.lightclientdata.org");

//     // // Set the execution rpc url
//     builder = builder.execution_rpc("https://eth-mainnet.g.alchemy.com/v2/XXXXX");

//     // // Set the checkpoint to the last known checkpoint
//     builder = builder.checkpoint(b256!(
//         "85e6151a246e8fdba36db27a0c7678a575346272fe978c9281e13a8b26cdfa68"
//     ));

//     // Set the rpc port
//     builder = builder.rpc_port(8545);

//     // Set the data dir
//     let temp_dir = env::temp_dir().join("helios");

//     if let Err(e) = create_directory_if_not_exists(&temp_dir) {
//         eprintln!("Failed to create helios directory: {}", e);
//     } else {
//         println!(
//             "Helios directory created or already exists at: {:?}",
//             temp_dir
//         );
//     }
//     builder = builder.data_dir(temp_dir);

//     // // Set the fallback service
//     builder = builder.fallback("https://sync-mainnet.beaconcha.in");

//     // // Enable lazy checkpoints
//     builder = builder.load_external_fallback();

//     // Build the client
//     let mut client: Client<FileDB> = builder.build().unwrap();
//     println!("Constructed client!");

//     env.new_string(format!("This is from Rust"))
//         .expect("Couldn't create Java string!")
//         .into_raw()
// }
