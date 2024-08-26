# ORE Boost

**ORE Boost is a proof-of-stake multiplier system for ORE mining.**

This program allows users to stake community tokens to earn a multiplier on their ORE mining rewards.


## API
- [`Consts`](api/src/consts.rs) – Program constants.
- [`Error`](api/src/error.rs) – Custom program errors.
- [`Event`](api/src/error.rs) – Custom program events.
- [`Instruction`](api/src/instruction.rs) – Declared instructions and arguments.

## Instructions
- [`Deposit`](program/src/deposit.rs) – Deposit ...
- [`Initialize`](program/src/initialize.rs) – Initializes the program and creates the global accounts.
- [`New`](program/src/new.rs) – New ...
- [`Open`](program/src/open.rs) – Open ...
- [`Update`](program/src/update.rs) – Update ...
- [`Withdraw`](program/src/withdraw.rs) – Withdraw ...

## State
 - [`Boost`](api/src/state/boost.rs) - ...
 - [`Config`](api/src/state/config.rs) – A singleton account which manages program-wide variables.
 - [`Stake`](api/src/state/stake.rs) - ...


## Tests

To run the test suite, use the Solana toolchain: 

```
cargo test-sbf
```

For line coverage, use llvm-cov:

```
cargo llvm-cov
```
