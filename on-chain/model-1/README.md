## Sollotto for lottery on Solana

In this repository, current implementation involves traditional lottery model.
Users will purchase ticket with their chosen number to participate in a lottery.
When it's ended, we store winning numbers to the program.
Depending on matched number count, we reward winners.
Winners will charge 65%, major part of rest will be distributed as charity.

Depending on the matched number count, we again split 65% to each winner.
3 matched: they will get ticket price back.
4 matched: these users will receive 2% / N of the total lottery amount.
5 matched: these users will receive 5% / N of the total lottery amount.
The final winners who matched all numbers will have the rest 65% - low tier prizes.
All "low tier" prizes are subtracted from the main prize pool of 65%. Thus, if no low tier winners, then they will get full 65%.

### Instructions

InitLottery: initalize lottery
PurchaseTicket: purchase ticket and provide his number
StoreWinningNumbers: owner stores(or generates) the randomly chosen winning numbers
RewardWinners: check winners and award prizes
UpdateCharity: update charity(1, 2, 3, 4)
UpdateSollottoWallets: update sollotto wallet settings

### Use of VRF for randomness

Current program is designed for the admin to generate winning numbers off-chain.
Solana ecosystem doesn't have a good source of randomness yet like Chainlink VRF on Ethereum.

There's switchboard VRF but its mechanism makes some troubles.
In chainlink VRF, the requested contract is called back from Chainlink after X blocks confirmation. But in switchboard, we need to parse the VRF account.
Due to this kind of issue, we need to take care of the workflow.
Current tryment is

- We store the VRF account pubkey when we init lottery.
- After lottery is done, Admin requests randomness to Switchboard VRF.
- Wait for enough confirmations are made.
- Admin calls the program to read randomness from VRF account.
- Reward winners

## Environment Setup

1. Install Rust from https://rustup.rs/
2. Install Solana v1.6.2 or later from https://docs.solana.com/cli/install-solana-cli-tools#use-solanas-install-tool

### Build and test for program compiled natively

```
$ cargo build
$ cargo test
```

### Build and test the program compiled for BPF

```
$ cargo build-bpf
$ cargo test-bpf
```
