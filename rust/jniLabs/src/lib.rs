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
use alloy::primitives::{utils, Address,B256};
use eyre::Result;

use helios::{client::ClientBuilder, config::networks::Network, prelude::*, types::BlockTag};
use std::{env, fs, io, path::Path, path::PathBuf, str::FromStr, thread, time::Duration};
use tokio::runtime::Runtime;

#[no_mangle]
pub extern "C" fn Java_com_example_mobilight_RustLib_getLatestBlockNumber(env: JNIEnv, _class: JClass) -> jstring {
    let rt = Runtime::new().unwrap();
    // which is a not best solution
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

    let result: Result<String, String> = rt.block_on(async {
        println!(
            "[Helios-Android-JNI]: Building client with data directory: {:?}",
            temp_dir
        );
        match create_client(temp_dir) {
            Ok(client) => {
                println!("[Helios-Android-JNI]: Client created successfully.");

                let res = match get_latest_block_number(client).await {
                    Ok(data) => {
                        println!(
                            "[Helios-Android-JNI]: Get Latest Block Numberance successfully."
                        );
                        data
                    }
                    Err(err_msg) => {
                        println!(
                            "[Helios-Android-JNI]: get Latest Block Number falied, {}",
                            err_msg
                        );
                        String::from(
                            "[Helios-Android-JNI]: Failed to get Latest Block Number with error",
                        )
                    }
                };

                Ok(res.to_string())
            }

            Err(err_msg) => {
                eprintln!("[Helios-Android-JNI]: Client creation error: {}", err_msg);
                Ok(
                    "[Helios-Android-JNI]: HSY Client created Failed due to dir creation error."
                        .to_string(),
                )
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


#[no_mangle]
pub extern "C" fn Java_com_example_mobilight_RustLib_getLatestCheckpoint(env: JNIEnv, _class: JClass) -> jstring {
    let rt = Runtime::new().unwrap();
    // Use async mode to get the balance
    let result: Result<String, String> = rt.block_on(async {
        let res = match get_latest_checkpoint().await {
            Ok(data) => {
                println!(
                    "[Helios-Android-JNI]: Get Lastest Block Numberance successfully."
                );
                data
            }
            Err(err_msg) => {
                println!(
                    "[Helios-Android-JNI]: get Lastest Block Number falied, {}",
                    err_msg
                );
                String::from(
                    "[Helios-Android-JNI]: Failed to get Lastest Block Number with error",
                )
            }
        };
        Ok(res.to_string())
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

#[no_mangle]
pub extern "C" fn Java_com_example_mobilight_RustLib_getLatestBlockNumberWithCP(
    mut env: JNIEnv,
    _class: JClass,
    input: JString,
) -> jstring {

    let rt = Runtime::new().unwrap();
    // which is a not best solution
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

    let checkpoint: String = env.get_string(&input).expect("Couldn't get Java string!").into();

    // Use async mode to get the balance
    let result: Result<String, String> = rt.block_on(async {
        println!(
            "[Helios-Android-JNI]: Building client with data directory: {:?}",
            temp_dir
        );
        match create_client_with_checkpoint(temp_dir,checkpoint) {
            Ok(client) => {
                println!("[Helios-Android-JNI]: Client created successfully.");

                let res = match get_latest_block_number(client).await {
                    Ok(data) => {
                        println!(
                            "[Helios-Android-JNI]: Get Latest Block Numberance successfully."
                        );
                        data
                    }
                    Err(err_msg) => {
                        println!(
                            "[Helios-Android-JNI]: get Latest Block Number falied, {}",
                            err_msg
                        );
                        String::from(
                            "[Helios-Android-JNI]: Failed to get Latest Block Number with error",
                        )
                    }
                };

                Ok(res.to_string())
            }

            Err(err_msg) => {
                eprintln!("[Helios-Android-JNI]: Client creation error: {}", err_msg);
                Ok(
                    "[Helios-Android-JNI]: HSY Client created Failed due to dir creation error."
                        .to_string(),
                )
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

#[no_mangle]
pub extern "C" fn Java_com_example_mobilight_RustLib_getBalance(
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
            "4cb357692b815ac5e8b282b83c2b4e53eae9b06c0a854981029f6f1c5fe5a128"
        ))
        .build()
        .map_err(|e| format!("Client build error: {}", e))?;

    Ok(client)
}


fn create_client_with_checkpoint(temp_dir: PathBuf, cp:String) -> Result<Client<FileDB>, String> {
    let untrusted_rpc_url = "https://eth-mainnet.g.alchemy.com/v2/yhqq51-m59s1dG5iJTq3cxDpxMwq39e-";

    let hex_string = if cp.starts_with("0x") {
        &cp[2..]
    } else {
        &cp
    };

    let bytes = hex::decode(hex_string).expect("Invalid hex string");

    let mut byte_array = [0u8; 32];
    byte_array.copy_from_slice(&bytes);

    let cp_value = B256::from(byte_array);

    let client = ClientBuilder::new()
        .network(Network::MAINNET)
        .consensus_rpc("https://www.lightclientdata.org")
        .execution_rpc(untrusted_rpc_url)
        .load_external_fallback() // this is important
        .data_dir(temp_dir)
        .checkpoint(cp_value)
        .build()
        .map_err(|e| format!("[Helios-Android-JNI]: Client build error: {}", e))?;

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
    let mut countdown = 14;
    while countdown > 0 {
        println!("Remaining time: {} seconds", countdown);
        thread::sleep(Duration::from_secs(1)); 
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

async fn get_latest_block_number(mut client: Client<FileDB>) -> Result<String, String> {
    println!("[Helios-Android-JNI]: Get Block by Number start");
    client
        .start()
        .await
        .map_err(|e| format!("[Helios-Android-JNI]: Client start error: {}", e))?;

    println!("[Helios-Android-JNI][Siyuan han Magic]: Sleep for a while zzzzzzzzzzzzz");

    // Magic number

    thread::sleep(Duration::from_secs(14));


    let block: Option<Block> = client
        .get_block_by_number(BlockTag::Latest, false)
        .await
        .map_err(|e| format!("[Helios-Android-JNI]: Get Block error: {}", e))?;

    match block {
        Some(value) => Ok(value.number.to_string()),
        None => Ok("no such block".to_string()),
    }
}


async fn get_latest_checkpoint() -> Result<String, String> {
    let cf = checkpoints::CheckpointFallback::new()
        .build()
        .await
        .unwrap();

    let mainnet_checkpoint = cf
        .fetch_latest_checkpoint(&networks::Network::MAINNET)
        .await
        .unwrap();
    println!("Fetched latest mainnet checkpoint: {mainnet_checkpoint}");

    Ok(mainnet_checkpoint.to_string())
}
