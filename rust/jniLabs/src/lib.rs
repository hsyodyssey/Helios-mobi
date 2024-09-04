use jni::JNIEnv;

// These objects are what you should use as arguments to your native
// function. They carry extra lifetime information to prevent them escaping
// this context and getting used after being GC'd.
use jni::objects::{JClass, JString};

// This is just a pointer. We'll be returning it from our function. We
// can't return one of the objects with lifetime information because the
// lifetime checker won't let us.
use jni::sys::jstring;

use std::{path::PathBuf, str::FromStr};
use helios::{client::ClientBuilder, config::networks::Network, types::BlockTag, prelude::*};
use alloy::primitives::{utils, Address};
use eyre::Result;
use tokio::runtime::Runtime;


// This keeps Rust from "mangling" the name and making it unique for this
// crate.
#[no_mangle]
pub extern "system" fn Java_HelloWorld_getBalance<'local>(env: JNIEnv<'local>,
// This is the class that owns our static method. It's not going to be used,
// but still must be present to match the expected signature of a static
// native method.
                                                     class: JClass<'local>,
                                                     input: JString<'local>)
                                                     -> jstring {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        match get_balance().await {
            Ok(balance) => {
                let output = env.new_string(format!("Hello this balance data is from Rust library; Address: 0x00000000219ab540356cBB839Cbe05303d7705Fa's balance is:  , {}!", balance))
                .expect("Couldn't create java string!");
                output.into_raw()
            }
            Err(err) => {
                println!("Error: {}", err);
                let output = env.new_string(format!("Hello Err, {}!", "Error"))
                .expect("Couldn't create java string!");
                output.into_raw()            
            }
        }
    }) 
}

async fn get_balance() -> Result<String> {
    let untrusted_rpc_url = "https://eth-mainnet.g.alchemy.com/v2/yhqq51-m59s1dG5iJTq3cxDpxMwq39e-";

    let mut client: Client<FileDB> = ClientBuilder::new()
        .network(Network::MAINNET)
        .consensus_rpc("https://www.lightclientdata.org")
        .execution_rpc(untrusted_rpc_url)
        .load_external_fallback()
        .data_dir(PathBuf::from("/tmp/helios"))
        .build()?;

    client.start().await?;
    client.wait_synced().await;


    let head_block_num = client.get_block_number().await?;
    let addr = Address::from_str("0x00000000219ab540356cBB839Cbe05303d7705Fa")?;
    let block = BlockTag::Latest;
    let balance = client.get_balance(addr, block).await?;

    // println!("synced up to block: {}", head_block_num);
    // println!("balance of deposit contract: {}", utils::format_ether(balance));

    Ok(utils::format_ether(balance))
}