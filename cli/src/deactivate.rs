use std::str::FromStr;

use ore_boost_api::state::Directory;
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

    pub async fn directory(&self) -> ClientResult<()> {
        let account = self.rpc_client.get_account_data(&ore_boost_api::state::directory_pda().0).await?;
        let directory = Directory::try_from_bytes(&account).unwrap();
        println!("directory: {:?}", directory);
        Ok(())
    }
}
