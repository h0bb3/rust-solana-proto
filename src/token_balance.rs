mod util;

use solana_client::rpc_client::RpcClient;
use spl_token::state::Account as SplAccount;
use spl_token::solana_program::program_pack::Pack;
use solana_program::pubkey::Pubkey;

use mpl_token_metadata::accounts::Metadata;


pub fn get_token_metadata(client: &RpcClient, token_mint: &Pubkey,) -> Result<Metadata, Box<dyn std::error::Error>> {
    // Fetch the metadata account data
    let (metadata_address, _) = Metadata::find_pda(token_mint);
    let metadata_account_data = client.get_account_data(&metadata_address)?;
    
    // Unpack the metadata
    let metadata = Metadata::safe_deserialize(&metadata_account_data)?;
    
    Ok(metadata)
}

pub fn get_associated_token_address(owner: &Pubkey, token_mint: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[
            owner.as_ref(),
            spl_token::id().as_ref(),
            token_mint.as_ref()
        ],
        &spl_associated_token_account::id()
    ).0
}

pub fn get_token_balance(
    client: &RpcClient,
    owner: &Pubkey,
    token_mint: &Pubkey,
) -> Result<u64, Box<dyn std::error::Error>> {
    // Compute the Associated Token Account address
    let associated_token_address = get_associated_token_address(owner, token_mint);
    // println!("Associated Token Account: {}", associated_token_address);

    // Fetch the token account data
    let account_data = client.get_account_data(&associated_token_address)?;
    // println!("Raw account data: {:?}", account_data);

    // Unpack the account data
    match SplAccount::unpack(&account_data) {
        Ok(account_state) => {
            println!("Token balance: {}", account_state.amount);
            Ok(account_state.amount)
        }
        Err(e) => {
            println!("Error unpacking account data: {:?}", e);
            Err(Box::new(e))
        }
    }
}

#[tokio::main]
async fn main() {
    // Fetch the RPC URL from the environment
    let client = RpcClient::new(util::devnet_rpc());

    // Get the owner's public key from environment variable
    let owner_pubkey: Pubkey = util::pubkey_from_env("SECRET_2_PUB");

    // Get the token mint public key from environment variable
    let token_mint_pubkey: Pubkey = util::pubkey_from_env("TOKEN_PUB");

    // Fetch and print the token balance
    match get_token_balance(&client, &owner_pubkey, &token_mint_pubkey) {
        Ok(balance) => {
            

            // Fetch and print the token metadata
            match get_token_metadata(&client, &token_mint_pubkey) {
                Ok(metadata) => {
                    // Print additional metadata here
                    println!("Account balance for token: {} is {} {}", metadata.name, balance, metadata.symbol);
                },
                Err(e) => {
                    println!("Token has no metadata but account balance: {}", balance);
                }
            }

        }
        Err(e) => {
            eprintln!("Failed to get token account balance: {}", e);
        }
    }

    
}