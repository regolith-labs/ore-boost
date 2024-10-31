use std::str::FromStr;

use ore_boost_api::state::{boost_pda, Boost};
use solana_client::client_error::Result as ClientResult;
use solana_sdk::{pubkey::Pubkey, signer::Signer};
use steel::AccountDeserialize;

use crate::{Cli, UpdateBoostArgs};

impl Cli {
    pub async fn update_boost(&self, args: UpdateBoostArgs) -> ClientResult<()> {
        let signer = self.signer();
        let mint = Pubkey::from_str(&args.mint).unwrap();
        let boost_address = boost_pda(mint).0;
        let Ok(data) = self.rpc_client.get_account_data(&boost_address).await else {
            println!("No boost found for mint {:?}", mint);
            return Ok(());
        };
        let boost = Boost::try_from_bytes(&data).unwrap();
        let ix = ore_boost_api::sdk::update_boost(
            signer.pubkey(),
            boost_address,
            args.expires_at.unwrap_or(boost.expires_at),
            args.multiplier.unwrap_or(boost.multiplier),
        );
        let sig = self.send_and_confirm(ix).await?;
        println!("sig: {}", sig);
        Ok(())
    }
}
