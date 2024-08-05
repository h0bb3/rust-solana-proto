mod util;

use solana_client::rpc_client::RpcClient;


#[tokio::main]
async fn main() {
    // Replace with your desired RPC endpoint (you can use solana mainnet, devnet, etc.)
    let rpc_url = util::devnet_rpc();
    let client = RpcClient::new(rpc_url.to_string());

    // Replace with the public key of the account you want to check
    let account_pubkey = util::pubkey_from_env("SECRET_PUB");

    match client.get_balance(&account_pubkey) {
        Ok(balance) => {
            println!("Account {} has balance: {}lamports", account_pubkey.to_string(), balance);
        },
        Err(e) => {
            eprintln!("Failed to get account balance: {}", e);
        }
    }
}
