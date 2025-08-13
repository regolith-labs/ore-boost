use ore_boost_api::{
    consts::RESERVE,
    instruction::UpdateBoost,
    state::{Boost, Config, Reserve},
};
use steel::*;

/// UpdateBoost updates the multiplier or expiry date on a boost.
pub fn process_update_boost(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = UpdateBoost::try_from_bytes(data)?;
    let weight = u64::from_le_bytes(args.weight);
    let expires_at = i64::from_le_bytes(args.expires_at);

    // Load accounts.
    let [signer_info, boost_info, config_info, config_tokens_info, reserve_info, reserve_tokens_info, token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    let boost = boost_info.as_account_mut::<Boost>(&ore_boost_api::ID)?;
    let config = config_info
        .as_account_mut::<Config>(&ore_boost_api::ID)?
        .assert_mut(|c| c.admin == *signer_info.key)?;
    config_tokens_info
        .is_writable()?
        .as_associated_token_account(config_info.key, &ore_api::consts::MINT_ADDRESS)?;
    reserve_info.as_account::<Reserve>(&ore_boost_api::ID)?;
    let reserve_tokens = reserve_tokens_info
        .is_writable()?
        .as_associated_token_account(reserve_info.key, &ore_api::consts::MINT_ADDRESS)?;
    token_program.is_program(&spl_token::ID)?;

    // Collect rewards
    boost.collect_rewards(config, &reserve_tokens);

    // Transfer aggregate boost rewards from reserve to config.
    transfer_signed(
        reserve_info,
        reserve_tokens_info,
        config_tokens_info,
        token_program,
        reserve_tokens.amount(),
        &[RESERVE],
    )?;

    // Update the boost multiplier.
    let old_weight = boost.weight;
    boost.weight = weight;
    boost.expires_at = expires_at;

    // Update the total weight.
    if config.boosts.contains(boost_info.key) {
        if weight > old_weight {
            config.total_weight += weight - old_weight;
        } else {
            config.total_weight -= old_weight - weight;
        }
    }

    Ok(())
}
