use ore_boost_api::state::{Boost, Config, Directory};
use steel::*;

/// Deactivate removes a boost from the directory.
pub fn process_deactivate(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer_info, boost_info, directory_info, config_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    boost_info.as_account::<Boost>(&ore_boost_api::ID)?;
    let directory = directory_info.as_account_mut::<Directory>(&ore_boost_api::ID)?;
    config_info
        .as_account::<Config>(&ore_boost_api::ID)?
        .assert(|c| c.authority == *signer_info.key)?;

    // Find and remove boost from directory
    for i in 0..(directory.len as usize) {
        if directory.boosts[i] == *boost_info.key {
            // Move last element to this position and decrease length
            directory.boosts[i] = directory.boosts[directory.len as usize - 1];
            directory.boosts[directory.len as usize - 1] = Pubkey::default();
            directory.len -= 1;
            break;
        }
    }

    Ok(())
} 