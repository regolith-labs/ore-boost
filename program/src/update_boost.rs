use ore_boost_api::{
    instruction::UpdateBoost,
    state::{Boost, Config},
};
use steel::*;

/// UpdateBoost updates the multiplier or expiry date on a boost.
pub fn process_update_boost(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = UpdateBoost::try_from_bytes(data)?;
    let weight = u64::from_le_bytes(args.weight);
    let expires_at = i64::from_le_bytes(args.expires_at);

    // Load accounts.
    let [signer_info, boost_info, config_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    let boost = boost_info.as_account_mut::<Boost>(&ore_boost_api::ID)?;
    let config = config_info
        .as_account_mut::<Config>(&ore_boost_api::ID)?
        .assert_mut(|c| c.admin == *signer_info.key)?;

    // TODO Handle global rewards factor.

    // Update the boost multiplier.
    let old_weight = boost.weight;
    boost.weight = weight;
    boost.expires_at = expires_at;

    // Update the total weight.
    if weight > old_weight {
        config.total_weight += weight - old_weight;
    } else {
        config.total_weight -= old_weight - weight;
    }

    Ok(())
}
