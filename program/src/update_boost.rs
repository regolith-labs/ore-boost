use ore_boost_api::{
    instruction::UpdateBoost,
    state::{Boost, Config},
};
use steel::*;

/// UpdateBoost updates the multiplier or expiry date on a boost.
pub fn process_update_boost(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = UpdateBoost::try_from_bytes(data)?;
    let multiplier = u64::from_le_bytes(args.multiplier);
    let expires_at = i64::from_le_bytes(args.expires_at);

    // Load accounts.
    let [signer_info, boost_info, config_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    let boost = boost_info.to_account_mut::<Boost>(&ore_boost_api::ID)?;
    config_info
        .to_account::<Config>(&ore_boost_api::ID)?
        .check(|c| c.authority == *signer_info.key)?;

    // Update the boost multiplier.
    boost.multiplier = multiplier;
    boost.expires_at = expires_at;

    Ok(())
}
