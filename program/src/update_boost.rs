use ore_boost_api::{
    instruction::UpdateBoost,
    loaders::{load_any_boost, load_config},
    state::{Boost, Config},
};
use ore_utils::*;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

/// UpdateBoost updates the multiplier or expiry date on a boost.
pub fn process_update_boost(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = UpdateBoost::try_from_bytes(data)?;
    let multiplier = u64::from_le_bytes(args.multiplier);
    let expires_at = i64::from_le_bytes(args.expires_at);

    // Load accounts.
    let [signer, boost_info, config_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    load_signer(signer)?;
    load_any_boost(boost_info, true)?;
    load_config(config_info, false)?;

    // Reject signer if not admin.
    let config_data = config_info.data.borrow();
    let config = Config::try_from_bytes(&config_data)?;
    if config.authority.ne(&signer.key) {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Update the boost multiplier.
    let mut boost_data = boost_info.data.borrow_mut();
    let boost = Boost::try_from_bytes_mut(&mut boost_data)?;
    boost.multiplier = multiplier;
    boost.expires_at = expires_at;

    Ok(())
}
