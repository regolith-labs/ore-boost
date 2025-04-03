use ore_api::state::Proof;
use ore_boost_api::prelude::*;
use steel::*;

/// Deposit adds tokens to a stake account.
pub fn process_deposit(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = Deposit::try_from_bytes(data)?;
    let amount = u64::from_le_bytes(args.amount);

    // Load accounts.
    let clock = Clock::get()?;
    let [signer_info, boost_info, config_info, deposits_info, mint_info, proof_info, rewards_info, sender_info, stake_info, treasury_info, treasury_tokens_info, ore_program, token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    let boost = boost_info
        .as_account_mut::<Boost>(&ore_boost_api::ID)?
        .assert_mut(|b| b.mint == *mint_info.key)?;
    let config = config_info.as_account_mut::<Config>(&ore_boost_api::ID)?;
    deposits_info
        .is_writable()?
        .as_associated_token_account(boost_info.key, &boost.mint)?;
    mint_info.as_mint()?;
    let proof = proof_info
        .as_account::<Proof>(&ore_api::ID)?
        .assert(|p| p.authority == *config_info.key)?;
    rewards_info
        .is_writable()?
        .as_associated_token_account(config_info.key, &ore_api::consts::MINT_ADDRESS)?;
    let sender = sender_info
        .is_writable()?
        .as_associated_token_account(signer_info.key, &boost.mint)?;
    let stake = stake_info
        .as_account_mut::<Stake>(&ore_boost_api::ID)?
        .assert_mut(|s| s.authority == *signer_info.key)?
        .assert_mut(|s| s.boost == *boost_info.key)?;
    ore_program.is_program(&ore_api::ID)?;
    token_program.is_program(&spl_token::ID)?;

    // Deposit into the boost.
    let amount = amount.min(sender.amount());
    stake.deposit(boost, &clock, config, &proof, amount);

    // Accumulate stake rewards.
    invoke_signed(
        &ore_api::sdk::claim(*config_info.key, *rewards_info.key, proof.balance),
        &[
            config_info.clone(),
            rewards_info.clone(),
            proof_info.clone(),
            treasury_info.clone(),
            treasury_tokens_info.clone(),
            token_program.clone(),
            ore_program.clone(),
        ],
        &ore_boost_api::ID,
        &[CONFIG],
    )?;

    // Transfer funds into deposit vault.
    transfer(
        signer_info,
        sender_info,
        deposits_info,
        token_program,
        amount,
    )?;

    Ok(())
}
