// FIXME: remove below line for imports hints
#![allow(unused_imports)]
use anyhow::Result;
use borsh::to_vec;
use clap::Parser;
use lazy_static::lazy_static;
use phoenix::program::{FillEvent, PhoenixMarketEvent};
use phoenix_sdk::sdk_client::SDKClient;
use serde::{Deserialize, Serialize};
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signature},
};
use std::{path::PathBuf, str::FromStr, sync::Arc};
use tokio::io::{AsyncWriteExt, BufWriter};
use tokio::sync::Mutex;
use tokio::{fs::File, task::JoinHandle};

lazy_static! {
    static ref PHOENIX: Pubkey =
        Pubkey::try_from("PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY").unwrap();
}

// TODO: #6 implement program argument parsing for `api-key`
const HELIUS_URL: &str = "https://rpc.helius.xyz/?api-key=a13cca14-7103-441a-876a-ba00bc01c8cf";

#[tokio::main]
async fn main() -> Result<()> {
    let payer = Keypair::new();
    // TODO: #5 implement storing events into files
    let phoenix_sdk = Arc::new(SDKClient::new(&payer, &HELIUS_URL).await?);
    // sanity check
    let block_hash = phoenix_sdk.client.get_latest_blockhash().await?;
    println!("Connected. Latest block hash: {block_hash}");
    let finalized_phoenix_transactions: Vec<String> = phoenix_sdk
        .client
        .get_signatures_for_address(&PHOENIX)
        .await?
        .into_iter()
        .map(|s| s.signature)
        .collect();
    assert!(finalized_phoenix_transactions.len() == 1000);
    // get phoenix events from each transaction and output it in encoded form
    // TODO: #3 speed this up
    for transaction in finalized_phoenix_transactions {
        for event in phoenix_sdk
            .core
            .parse_events_from_transaction(
                &phoenix_sdk
                    .client
                    .get_transaction(&Signature::from_str(transaction.as_ref())?)
                    .await?,
            )
            .unwrap()
            .into_iter()
            // TODO: #1 select only `phoenix::program::FillEvent`
            .fold(Vec::new(), |mut accum, e| {
                accum.extend_from_slice(e.batch.as_ref());
                accum
            })
        {
            // TODO: #2 implement json encoding for `FillEvent` before printing it out
            println!("{}", &hex::encode(to_vec(&event)?));
        }
    }

    println!("All done.");
    Ok(())
}

// TODO: #4 implement test
