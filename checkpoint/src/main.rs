mod checkpoint;
mod client;
mod error;
mod lookup_tables;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let mint = mint()?;
    let client = client::Client::new()?;
    checkpoint::run(&client, &mint).await?;
    Ok(())
}

fn mint() -> anyhow::Result<solana_sdk::pubkey::Pubkey> {
    let str = std::env::var("MINT")?;
    use std::str::FromStr;
    let pubkey = solana_sdk::pubkey::Pubkey::from_str(str.as_str())?;
    Ok(pubkey)
}
