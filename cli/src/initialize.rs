use solana_client::client_error::Result as ClientResult;
use solana_sdk::signer::Signer;

use crate::Cli;

impl Cli {
    pub async fn initialize(&self) -> ClientResult<()> {
        let signer = self.signer();
        let ix = ore_boost_api::sdk::initialize(signer.pubkey());
        let sig = self.send_and_confirm(ix).await?;
        println!("sig: {}", sig);
        Ok(())
    }
}
