use ore_boost_api::prelude::*;
use solana_program::{address_lookup_table, log::sol_log};
use steel::*;

pub fn process_create_stake_lookup_table(
    accounts: &[AccountInfo<'_>],
    data: &[u8],
) -> ProgramResult {
    // Parse args.
    let args = CreateStakeLookupTable::try_from_bytes(data)?;

    // Load accounts.
    let [signer_info, stake_lookup_table_info, lookup_table_info, boost_info, system_program, lookup_table_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    boost_info.as_account::<Boost>(&ore_boost_api::ID)?;
    let stake_lookup_table_seeds = vec![
        STAKE_LOOKUP_TABLE,
        boost_info.key.as_ref(),
        args.lut_id.as_slice(),
    ];
    sol_log("stake lookup");
    stake_lookup_table_info
        .is_writable()?
        .is_empty()?
        .has_seeds(stake_lookup_table_seeds.as_slice(), &ore_boost_api::ID)?;
    sol_log("lookup");
    lookup_table_info.is_writable()?.is_empty()?.has_seeds(
        &[
            stake_lookup_table_info.key.as_ref(),
            args.lut_slot.as_slice(),
        ],
        &address_lookup_table::program::ID,
    )?;

    sol_log("create stake lookup");
    // Initalize stake lookup table account.
    create_account::<StakeLookupTable>(
        stake_lookup_table_info,
        system_program,
        signer_info,
        &ore_boost_api::ID,
        stake_lookup_table_seeds.as_slice(),
    )?;
    let stake_lookup_table =
        stake_lookup_table_info.as_account_mut::<StakeLookupTable>(&ore_boost_api::ID)?;
    stake_lookup_table.lookup_table = *lookup_table_info.key;

    sol_log("create lookup");
    // Initalize lookup table account.
    let lut_slot = u64::from_le_bytes(args.lut_slot);
    let (create_lookup_table_ix, _) = address_lookup_table::instruction::create_lookup_table_signed(
        *stake_lookup_table_info.key,
        *signer_info.key,
        lut_slot,
    );
    let create_lookup_table_accounts = vec![
        lookup_table_info.clone(),
        stake_lookup_table_info.clone(),
        signer_info.clone(),
        system_program.clone(),
        lookup_table_program.clone(),
    ];
    invoke_signed_with_bump(
        &create_lookup_table_ix,
        create_lookup_table_accounts.as_slice(),
        stake_lookup_table_seeds.as_slice(),
        args.stake_bump,
    )?;

    Ok(())
}
