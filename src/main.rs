mod dex;
mod utils;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use spl_token_client::client::{ProgramClient, ProgramRpcClient, ProgramRpcClientSendTransaction};
use std::{sync::Arc, str::FromStr};
use solana_sdk::signature::Keypair;

#[tokio::main]
async fn main() {

    let kapital: u64 = 100000000; // 0,1 SOL

    let private_key_str: &str = "3Detg1HDHh4RgKmTJnZMUubatoxgWGm37wqPJ6kJLpbnQ9yfh1q7MHjRHCwvHMT7bcD6ZqT24P8ve9UGb4xmLpzH";
    let addresses: Vec<&str> = ["8wXtPeU6557ETkp9WHFY1n1EcU6NxDvbAggHGsMYiHsB"].to_vec();
    let rpc: &str = "http://localhost:8899";

    

    let mut handles = vec![];

    for address in addresses {
        let handle = tokio::spawn(async move {
            swap_io_ray(private_key_str, rpc).await;
        });
        handles.push(handle);
    }

    futures::future::join_all(handles).await;
}

async fn swap_io_ray(private_key_str: &str, rpc: &str, token_out: Pubkey, amount_in: u64, min_amount_out: u64) {

    let token_in: Pubkey = pubkey!("So11111111111111111111111111111111111111112");

    let private_key_bytes = bs58::decode(private_key_str).into_vec().unwrap();
    let keypair = match Keypair::from_bytes(&private_key_bytes) {
        Ok(keypair) => keypair,
        Err(_) => panic!("Неизвестная ошибка"),
    };
    let payer: Arc<Keypair> = Arc::new(keypair);

    let rpc_cl = RpcClient::new(rpc.to_string());
    let program_client = Arc::new(ProgramRpcClient::new(Arc::new(rpc_cl), ProgramRpcClientSendTransaction));

    let client = crate::dex::raydium::RaydiumCliemt::new(payer, program_client);

    client.swap(token_in, token_out, amount_in, min_amount_out).await;
}