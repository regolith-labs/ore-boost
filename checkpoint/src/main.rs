mod checkpoint;
mod client;
mod error;
mod lookup_tables;
mod notifier;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let client = client::Client::new()?;
    let client = std::sync::Arc::new(client);
    match mint() {
        Ok(mint) => {
            log::info!("mint found: {}", mint);
            log::info!("running single boost checkpoint");
            checkpoint::run(client.as_ref(), &mint).await?;
        }
        Err(_) => {
            log::info!("no mint found, running all checkpoints");
            checkpoint::run_all(client).await?;
        }
    }
    Ok(())
}

fn mint() -> anyhow::Result<solana_sdk::pubkey::Pubkey> {
    use solana_sdk::pubkey::Pubkey;
    use std::str::FromStr;
    let mint = std::env::var("MINT").map_err(|err| anyhow::anyhow!(err));
    mint.and_then(|string| Pubkey::from_str(string.as_str()).map_err(|err| anyhow::anyhow!(err)))
}
