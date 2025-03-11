use std::str::FromStr;

use ore_boost_api::state::Config;
use solana_client::client_error::Result as ClientResult;
use solana_sdk::signer::Signer;
use steel::{AccountDeserialize, Pubkey};

use crate::{Cli, DeactivateArgs};

impl Cli {
    pub async fn deactivate(&self, args: DeactivateArgs) -> ClientResult<()> {
        let mint = Pubkey::from_str(&args.mint).unwrap();
        let signer = self.signer();
        let ix = ore_boost_api::sdk::deactivate(signer.pubkey(), mint);
        let sig = self.send_and_confirm(ix).await?;
        println!("sig: {}", sig);
        Ok(())
    }

    pub async fn config(&self) -> ClientResult<()> {
        let account: Vec<u8> = self
            .rpc_client
            .get_account_data(&ore_boost_api::state::config_pda().0)
            .await?;
        let config = Config::try_from_bytes(&account).unwrap();
        println!("config: {:?}", config);
        Ok(())
    }
}
