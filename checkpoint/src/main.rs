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
    checkpoint::run_all(client).await?;
    Ok(())
}
