use ore_boost_api::state::*;
use steel::*;

/// Activate adds a boost to the directory.
pub fn process_activate(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer_info, boost_info, directory_info, config_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    boost_info.as_account::<Boost>(&ore_boost_api::ID)?;
    let directory = directory_info
        .as_account_mut::<Directory>(&ore_boost_api::ID)?
        .assert_mut(|d| d.len < 256)?;
    config_info
        .as_account::<Config>(&ore_boost_api::ID)?
        .assert(|c| c.authority == *signer_info.key)?;
    
    // Check if boost is already in directory
    if directory.boosts.contains(boost_info.key) {
        return Ok(());
    }

    // Add boost to directory if not found
    directory.boosts[directory.len as usize] = *boost_info.key;
    directory.len += 1;

    Ok(())
} 