use ore_boost_api::{instruction::UpdateAdmin, state::Config};
use steel::*;

/// UpdateAdmin updates the program admin authority.
pub fn process_update_admin(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = UpdateAdmin::try_from_bytes(data)?;

    // Load accounts.
    let [signer_info, config_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    let config = config_info
        .as_account_mut::<Config>(&ore_boost_api::ID)?
        .assert_mut(|c| c.admin == *signer_info.key)?;

    // Update the admin.
    config.admin = args.new_admin;

    Ok(())
}
