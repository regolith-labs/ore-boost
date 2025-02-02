use std::str::FromStr;

use ore_boost_api::state::{find_stake_lookup_table_id, Boost, Checkpoint, StakeLookupTable};
use solana_sdk::signer::Signer;
use steel::{AccountDeserialize, Pubkey};

use crate::{args::OpenArgs, Cli};

impl Cli {
    pub async fn open(&self, args: OpenArgs) {
        let signer = self.signer();
        let rpc_client = &self.rpc_client;
        let mint = Pubkey::from_str(args.mint.as_str()).unwrap();
        let (boost_pda, _) = ore_boost_api::state::boost_pda(mint);
        let boost_data = self
            .rpc_client
            .as_ref()
            .get_account_data(&boost_pda)
            .await
            .unwrap();
        let boost = Boost::try_from_bytes(boost_data.as_slice()).unwrap();
        let lut_id = find_stake_lookup_table_id(boost.total_stakers);
        let (stake_lookup_table_pda, _) =
            ore_boost_api::state::stake_lookup_table_pda(boost_pda, lut_id);
        let stake_lookup_table_data = rpc_client
            .as_ref()
            .get_account_data(&stake_lookup_table_pda)
            .await
            .unwrap();
        let stake_lookup_table =
            StakeLookupTable::try_from_bytes(stake_lookup_table_data.as_slice()).unwrap();
        let lookup_table_pda = stake_lookup_table.lookup_table;
        let ix = ore_boost_api::sdk::open(
            signer.pubkey(),
            signer.pubkey(),
            mint,
            stake_lookup_table_pda,
            lookup_table_pda,
        );
        let sig = self.send_and_confirm(ix).await;
        println!("sig: {:?}", sig);
    }
}
