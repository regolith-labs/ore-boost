use std::str::FromStr;

use solana_sdk::signer::Signer;
use steel::Pubkey;

use crate::{args::CreateStakeLookupTableArgs, Cli};

impl Cli {
    pub async fn create_stake_lookup_table(&self, args: CreateStakeLookupTableArgs) {
        let signer = self.signer();
        let rpc_client = &self.rpc_client;
        let slot = rpc_client.as_ref().get_slot().await.unwrap();
        let boost = Pubkey::from_str(args.boost.as_str()).unwrap();
        let ix = ore_boost_api::sdk::create_stake_lookup_table(
            signer.pubkey(),
            boost,
            args.lut_id,
            slot,
        );
        let sig = self.send_and_confirm(ix).await;
        println!("sig: {:?}", sig);
    }
}
