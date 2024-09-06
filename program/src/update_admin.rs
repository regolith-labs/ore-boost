use ore_boost_api::{instruction::UpdateAdmin, loaders::load_config, state::Config};
use ore_utils::*;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

/// UpdateAdmin updates the program admin authority.
pub fn process_update_admin(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = UpdateAdmin::try_from_bytes(data)?;

    // Load accounts.
    let [signer, config_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    load_signer(signer)?;
    load_config(config_info, false)?;

    // Reject signer if not admin.
    let mut config_data = config_info.data.borrow_mut();
    let config = Config::try_from_bytes_mut(&mut config_data)?;
    if config.authority.ne(&signer.key) {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Update the admin.
    config.authority = args.new_admin;

    Ok(())
}
