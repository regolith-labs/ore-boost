use ore_api::state::Proof;
use ore_boost_api::{consts::BOOST, instruction::Payout, state::Boost};
use steel::*;

/// Payout ...
pub fn process_payout(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = Payout::try_from_bytes(data)?;
    let amount = u64::from_le_bytes(args.amount);

    // Load accounts.
    let [signer_info, boost_info, boost_rewards_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?.as_account::<Proof>(&ore_api::ID)?;
    let boost = boost_info
        .as_account_mut::<Boost>(&ore_boost_api::ID)?
        .assert_mut(|b| b.proof == *signer_info.key)? // Can only be called by the proof this boost is reserved for
        .assert_mut(|b| b.locked == 0)?; // Don't allow boost to be used until checkpoint is complete
    boost_rewards_info
        .is_writable()?
        .as_token_account()?
        .assert(|t| t.mint == ore_api::consts::MINT_ADDRESS)?
        .assert(|t| t.owner == *boost_info.key)?;

    // Increment boost rewards.
    boost.rewards = boost.rewards.checked_add(amount).unwrap();

    // Claim yield as the boost.
    invoke_signed(
        &ore_api::sdk::claim(
            *boost_info.key, 
            *boost_rewards_info.key, 
            u64::MAX
        ), 
        accounts, 
        &ore_api::ID, 
        &[BOOST, boost_info.key.as_ref()]
    )?;

    Ok(())
}
