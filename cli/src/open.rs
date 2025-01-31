use std::str::FromStr;

use ore_boost_api::state::{find_stake_lookup_table_id, Checkpoint, StakeLookupTable};
use solana_sdk::signer::Signer;
use steel::{AccountDeserialize, Pubkey};

use crate::{args::OpenArgs, Cli};

impl Cli {
    pub async fn open(&self, args: OpenArgs) {
        let signer = self.signer();
        let rpc_client = &self.rpc_client;
        let mint = Pubkey::from_str(args.mint.as_str()).unwrap();
        let (boost_pda, _) = ore_boost_api::state::boost_pda(mint);
        let (checkpoint_pda, _) = ore_boost_api::state::checkpoint_pda(boost_pda);
        let checkpoint_data = rpc_client
            .as_ref()
            .get_account_data(&checkpoint_pda)
            .await
            .unwrap();
        let checkpoint = Checkpoint::try_from_bytes(checkpoint_data.as_slice()).unwrap();
        let lut_id = find_stake_lookup_table_id(checkpoint.total_stakers + 1);
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
