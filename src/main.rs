mod dex;
mod utils;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use spl_token_client::client::{ProgramRpcClient, ProgramRpcClientSendTransaction};
use std::{sync::Arc, str::FromStr};
use solana_sdk::signature::Keypair;

#[tokio::main]
async fn main() {

    let kapital: u64 = 100000000 - 105000; // 0,1 SOL

    let private_key_str: &str = "1";
    let addresses: Vec<&str> = ["8wXtPeU6557ETkp9WHFY1n1EcU6NxDvbAggHGsMYiHsB"].to_vec();
    let rpc: &str = "http://localhost:8899";

    let min_amount_out: u64 = 0;

    for address in addresses {
        let res: Result<solana_sdk::signature::Signature, eyre::Error> = swap_io_ray(private_key_str, rpc, address, kapital, min_amount_out).await;

        match res {
            Ok(signature) => {
                println!("Tx | {}", signature);
            },
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}

async fn swap_io_ray(private_key_str: &str, rpc: &str, address: &str, amount_in: u64, min_amount_out: u64) -> Result<solana_sdk::signature::Signature, eyre::Error> {

    let token_in: Pubkey = pubkey!("So11111111111111111111111111111111111111112");

    let token_out: Pubkey = solana_sdk::pubkey::Pubkey::from_str(address).unwrap();

    let private_key_bytes = bs58::decode(private_key_str).into_vec().unwrap();
    let keypair = match Keypair::from_bytes(&private_key_bytes) {
        Ok(keypair) => keypair,
        Err(_) => panic!("Неизвестная ошибка"),
    };
    let payer: Arc<Keypair> = Arc::new(keypair);

    let rpc_cl = RpcClient::new(rpc.to_string());
    let program_client = Arc::new(ProgramRpcClient::new(Arc::new(rpc_cl), ProgramRpcClientSendTransaction));

    let client = crate::dex::raydium::RaydiumCliemt::new(payer, program_client);

    let res: Result<solana_sdk::signature::Signature, eyre::Error> = client.swap(token_in, token_out, amount_in, min_amount_out).await;

    res
}
