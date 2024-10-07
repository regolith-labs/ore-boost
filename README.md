# ORE Boost

**ORE Boost is a staking program to earn ORE mining multipliers.** 


## API
- [`Consts`](api/src/consts.rs) – Program constants.
- [`Error`](api/src/error.rs) – Custom program errors.
- [`Event`](api/src/error.rs) – Custom program events.
- [`Instruction`](api/src/instruction.rs) – Declared instructions and arguments.

## Instructions
- [`Close`](program/src/close.rs) – Closes a stake account.
- [`Deposit`](program/src/deposit.rs) – Deposits tokens into a stake account.
- [`Initialize`](program/src/initialize.rs) – Initializes the program and creates the global accounts.
- [`New`](program/src/new.rs) – Creates a new boost account.
- [`Open`](program/src/open.rs) – Opens a new stake account.
- [`UpdateAdmin`](program/src/update_admin.rs) – Updates the admin key.
- [`UpdateBoost`](program/src/update_boost.rs) – Updates the data on a boost.
- [`Withdraw`](program/src/withdraw.rs) – Withdraws tokens from a stake account.

## State
 - [`Boost`](api/src/state/boost.rs) - An account (1 per mint) which records how much of a multiplier should be paid out for staked tokens of a given mint.
 - [`Config`](api/src/state/config.rs) – A singleton account which manages program-wide variables.
 - [`Stake`](api/src/state/stake.rs) - An account (1 per user per mint) which records how many tokens of a given mint a user has staked. 


## Tests

To run the test suite, use the Solana toolchain: 

```
cargo test-sbf
```

For line coverage, use llvm-cov:

```
cargo llvm-cov
```
