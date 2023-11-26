use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::commitment_config::CommitmentConfig;
use std::str::FromStr;

fn main() {
    let address = std::env::args()
        .nth(1)
        .expect("You must provide an address");

    println!("Fetching balance for address: {}", address);

    let rpc_client = RpcClient::new("https://api.mainnet-beta.solana.com");
    let pub_key = Pubkey::from_str(&address)
        .expect("unable to create pubkey");

    let balance = rpc_client
        .get_balance_with_commitment(&pub_key, CommitmentConfig::processed())
        .expect("unable to fetch balance");

    let balance = balance.value as f64 / 1e9;
    println!("{}", balance)
}
