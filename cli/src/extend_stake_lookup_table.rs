use std::str::FromStr;

use ore_boost_api::{
    sdk::extend_stake_lookup_table,
    state::{find_stake_lookup_table_id, Stake, StakeLookupTable},
};
use solana_sdk::signer::Signer;
use steel::{AccountDeserialize, Pubkey};

use crate::{args::ExtendStakeLookupTableArgs, Cli};

impl Cli {
    pub async fn extend_stake_lookup_table(&self, args: ExtendStakeLookupTableArgs) {
        let signer = self.signer();
        let rpc_client = &self.rpc_client;
        let boost = Pubkey::from_str(args.boost.as_str()).unwrap();
        let (stake_pda, _) = ore_boost_api::state::stake_pda(signer.pubkey(), boost);
        let stake_data = rpc_client
            .as_ref()
            .get_account_data(&stake_pda)
            .await
            .unwrap();
        let stake = Stake::try_from_bytes(stake_data.as_slice()).unwrap();
        let lut_id = find_stake_lookup_table_id(stake.id);
        let (stake_lookup_table_pda, _) =
            ore_boost_api::state::stake_lookup_table_pda(boost, lut_id);
        let stake_lookup_table_data = rpc_client
            .as_ref()
            .get_account_data(&stake_lookup_table_pda)
            .await
            .unwrap();
        let stake_lookup_table =
            StakeLookupTable::try_from_bytes(stake_lookup_table_data.as_slice()).unwrap();
        let lookup_table_pda = stake_lookup_table.lookup_table;
        let ix = extend_stake_lookup_table(
            signer.pubkey(),
            boost,
            stake_pda,
            stake_lookup_table_pda,
            lookup_table_pda,
        );
        let sig = self.send_and_confirm(ix).await;
        println!("sig: {:?}", sig);
    }
}
