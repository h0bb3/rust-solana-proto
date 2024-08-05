use solana_program::pubkey::Pubkey;
use std::env;
use dotenv::dotenv;
use std::str::FromStr;

pub fn pubkey_from_env(env_key: &str) -> Pubkey {
    dotenv().ok();

    let value = env::var(env_key).expect(&format!("{} must be set in environment", env_key));
    Pubkey::from_str(&value).expect("Invalid public key")
}

pub fn mainnet_rpc() -> String {
    get_value_from_env("MAINNET_RPC", "https://api.mainnet-beta.solana.com")
}

pub fn devnet_rpc() -> String {
    get_value_from_env("DEVNET_RPC", "https://api.devnet.solana.com")
}

fn get_value_from_env(key: &str, default: &str) -> String {
    match env::var(key) {
        Ok(value) => value,
        Err(_) => default.to_string(),
    }
}