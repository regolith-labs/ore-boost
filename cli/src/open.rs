use std::str::FromStr;

use solana_sdk::signer::Signer;
use steel::Pubkey;

use crate::{args::OpenArgs, Cli};

impl Cli {
    pub async fn open(&self, args: OpenArgs) {
        let signer = self.signer();
        let mint = Pubkey::from_str(&args.mint).unwrap();
        let ix = ore_boost_api::sdk::open(signer.pubkey(), signer.pubkey(), mint);
        let (boost_pda, _) = ore_boost_api::state::boost_pda(mint);
        let (stake_pda, _) = ore_boost_api::state::stake_pda(signer.pubkey(), boost_pda);
        println!("{:?}", boost_pda);
        println!("{:?}", stake_pda);
        let sig = self.send_and_confirm(ix).await.unwrap();
        println!("sig: {}", sig);
    }
}
