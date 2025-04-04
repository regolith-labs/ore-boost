
# Migration plan

- [ ] Deploy ore program migration state
    - [ ] Ignore all boosts and boost accounts

- [ ] Clear boost proof balances
    - [ ] Claim from all boosts
    - [ ] Verify all boost proof balances are zero

- [ ] Deploy boost program migration state 
    - [ ] Disable claims
    - [ ] Disable withdraws
    - [ ] Disable deposits

- [ ] Migrate config account
    - [ ] Migrate config data and reclaim rent from buffer
    - [ ] Create proof for config account
    - [ ] Create token account to hold all boost yield

- [ ] Migrate boost rewards
    - [ ] Transfer boost rewards from the current atas to global pool
    - [ ] Verify global pool has enough reserves to cover all boost debts
    - [ ] Close ata and return rent to admin

- [ ] Migrate boost accounts
    - [ ] Migrate boost data and reclaim rent from buffer
    - [ ] Verify global pool has enough reserves to cover all boost debts

- [ ] Update boost weights
    - [ ] Set boost weights to appropriate values given current rewards split
    - [ ] Ensure global boost take rate is =50%

- [ ] Renable boost program
    - [ ] Enable claims
    - [ ] Enable withdraws
    - [ ] Enable deposits

- [ ] Renable ore program boosts
    - [ ] Deploy new ore program that pays global take rate to config account

    