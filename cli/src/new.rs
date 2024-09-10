use std::str::FromStr;

use solana_client::client_error::Result as ClientResult;
use solana_sdk::{pubkey::Pubkey, signer::Signer};

use crate::{args::NewArgs, Cli};

impl Cli {
    pub async fn new_boost(&self, args: NewArgs) -> ClientResult<()> {
        let signer = self.signer();
        let mint = Pubkey::from_str(&args.mint).unwrap();
        let ix = ore_boost_api::sdk::new(signer.pubkey(), mint, args.expires_at, args.multiplier);
        let sig = self.send_and_confirm(ix).await?;
        println!("sig: {}", sig);
        Ok(())
    }
}
