<p align="center">
  <a href="https://app.sollotto.io/">
    <img alt="Metaplex" src="https://app.sollotto.io/static/media/SolLotto-logo-horizontal.b4b49b1a.png" width="250" />
  </a>
</p>

# About us

SolLotto is the first decentralized peer-to-peer lottery system built on the Solana blockchain.

We believe a community-driven approach to effective altruism will result in the most impact globally, which is why we're introducing a lottery system that utilizes verifiable community-consensus to dynamically allocate the resources where the community believes they will do the most good each week.

### Model 1 (Traditional “Lottery” Giveaway Model)

Donate for a ticket, pick 6 numbers, vote for a charity, if your numbers match the 6 randomly selected winning numbers then you win.

### Model 2 (Pooled Giveaway Model)

Stake supported tokens, have a chance to win the prize pool generated from staking rewards based on how much of the staking pool you contributed.

### Model 3 (Launchpad Pool Model)

Stake supported tokens, have a chance to win a prize pool donated by an up and coming Solana project.

### Model 4 (Lifetime “Lottery” Giveaway Model)

Chances to win are increased by playing the traditional lottery model

### Model 5 (Fixed-Quantity Giveaway Model)(not implemented)

Only holders of the platform token are eligible to purchase a single ticket for each token held.

# Usage

## Installation

```bash
$ git clone https://github.com/sollotto-io/sollotto-backend.git
$ cd solloto-backend
$ npm install
```

### Setup enviroment variables

In order to start the app go to the `.env` file and proceed to fill the environment variables.

For setting up a development environment you will need to start locally the frontend server included in [sollotto frontend](https://github.com/sollotto-io/sollotto-frontend.git) repo.

#### Enviroment Variables

```
SOLANA_INIT_LOTTERY_PROGRAM = ""
SOLANA_NETWORK = ""
LOTTERY_ID = ""
MONGO_DB = ""
JWT_SECRET= ""
SECRET_KEY =""
HOLDING_WALLET_SECRETKEY = ""
REWARD_WALLET_PUBLIC_KEY = ""
SLOT_HOLDER_REWARDS_PUBLIC_KEY = ""
SOLLOTTO_LABS_PUBLIC_KEY = ""
```

- **SOLANA_INIT_PROGRAM:** Is the address for the on-chain model 1 lottery initialization. The on-chain model 1 program source code can be found in the [sollotto backend](https://github.com/sollotto-io/sollotto-backend.git) in the **feature/model4** branch

- **SOLANA_NETWORK:** The Solana network in which the program operates, for example: `https://api.devnet.solana.com/`

- **LOTTERY_ID:** The MongoDB id for the model 1 Lottery (You will need to create a document, you can see the schema for the model 1 lottery in `/models`).

- **MONGO_DB**: MongoDB connection string.

- **JWT_SECRET:** The secret for the JWT generation

- **SECRET_KEY:** Secret key for encrypting information

- **HOLDING_WALLET_SECRETKEY:** Secret key for the holding wallet

- **REWARD_WALLET_PUBLIC_KEY:** Public key for the reward wallet

- **SLOT_HOLDER_REWARDS_PUBLIC_KEY:** Public key for the slot holder rewards

- **SOLLOTTO_LABS_PUBLIC_KEY:** Public key for your project wallet

## Start the app

```bash
$ npm start
```

Once is started you can go to `http://localhost:5000/graphql` to see the app running

# Devops

This repo includes integration with **Github Actions** and **AWS Codeploy & Code Pipeline** for CI/CD. You can see the scripts integration in the `.github/workflows` and `/scripts` folders
