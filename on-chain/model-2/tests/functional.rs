use solana_program::{
    hash::Hash,
    instruction::InstructionError,
    native_token::sol_to_lamports,
    program_pack::Pack,
    system_instruction::{self},
};
use solana_program_test::*;
use solana_sdk::{
    signature::Keypair, system_transaction, transaction::TransactionError,
    transport::TransportError,
};
use sollotto_model_2::{
    processor::id,
    processor::Processor,
    state::{LotteryData, LotteryResultData},
};
use spl_token::state::{Account, Mint};
use {
    solana_program::pubkey::Pubkey,
    solana_sdk::{signature::Signer, transaction::Transaction},
};

// Helper functions

async fn initialize_lottery(
    banks_client: &mut BanksClient,
    payer: &Keypair,
    recent_blockhash: &Hash,
    lottery_data_rent: u64,
    mint_rent: u64,
    token_account_rent: u64,
    staking_pool_wallet: &Pubkey,
    staking_pool_token_mint: &Keypair,
    staking_pool_token_account: &Keypair,
    rewards_wallet: &Pubkey,
    slot_holders_rewards_wallet: &Pubkey,
    sollotto_labs_wallet: &Pubkey,
    lottery_authority: &Keypair,
) -> Result<(), TransportError> {
    let mut transaction = Transaction::new_with_payer(
        &[
            system_instruction::create_account(
                &payer.pubkey(),
                &lottery_authority.pubkey(),
                lottery_data_rent,
                sollotto_model_2::state::LotteryData::LEN as u64,
                &id(),
            ),
            system_instruction::create_account(
                &payer.pubkey(),
                &staking_pool_token_mint.pubkey(),
                mint_rent,
                spl_token::state::Mint::LEN as u64,
                &spl_token::id(),
            ),
            system_instruction::create_account(
                &payer.pubkey(),
                &staking_pool_token_account.pubkey(),
                token_account_rent,
                spl_token::state::Account::LEN as u64,
                &spl_token::id(),
            ),
            sollotto_model_2::instruction::initialize_lottery(
                &id(),
                staking_pool_wallet,
                &staking_pool_token_mint.pubkey(),
                &staking_pool_token_account.pubkey(),
                rewards_wallet,
                slot_holders_rewards_wallet,
                sollotto_labs_wallet,
                &lottery_authority.pubkey(),
            )
            .unwrap(),
        ],
        Some(&payer.pubkey()),
    );
    transaction.sign(
        &[
            payer,
            lottery_authority,
            staking_pool_token_mint,
            staking_pool_token_account,
        ],
        *recent_blockhash,
    );
    banks_client.process_transaction(transaction).await?;
    Ok(())
}

async fn deposit(
    banks_client: &mut BanksClient,
    payer: &Keypair,
    recent_blockhash: &Hash,
    amount: u64,
    staking_pool_token_mint: &Pubkey,
    user_staking_pool_token_account: &Pubkey,
    staking_pool_wallet: &Pubkey,
    user_authority: &Keypair,
    lottery_authority: &Keypair,
) -> Result<(), TransportError> {
    let mut transaction = Transaction::new_with_payer(
        &[sollotto_model_2::instruction::deposit(
            &id(),
            amount,
            staking_pool_token_mint,
            user_staking_pool_token_account,
            staking_pool_wallet,
            &user_authority.pubkey(),
            &lottery_authority.pubkey(),
        )
        .unwrap()],
        Some(&payer.pubkey()),
    );
    transaction.sign(
        &[payer, user_authority, lottery_authority],
        *recent_blockhash,
    );
    banks_client.process_transaction(transaction).await?;
    Ok(())
}

async fn undeposit(
    banks_client: &mut BanksClient,
    payer: &Keypair,
    recent_blockhash: &Hash,
    amount: u64,
    staking_pool_token_mint: &Pubkey,
    user_staking_pool_token_account: &Pubkey,
    staking_pool_wallet: &Keypair,
    user_authority: &Keypair,
    lottery_authority: &Keypair,
) -> Result<(), TransportError> {
    let mut transaction = Transaction::new_with_payer(
        &[sollotto_model_2::instruction::undeposit(
            &id(),
            amount,
            staking_pool_token_mint,
            user_staking_pool_token_account,
            &staking_pool_wallet.pubkey(),
            &user_authority.pubkey(),
            &lottery_authority.pubkey(),
        )
        .unwrap()],
        Some(&payer.pubkey()),
    );
    transaction.sign(
        &[
            payer,
            user_authority,
            lottery_authority,
            staking_pool_wallet,
        ],
        *recent_blockhash,
    );
    banks_client.process_transaction(transaction).await?;
    Ok(())
}

async fn reward_winner(
    banks_client: &mut BanksClient,
    payer: &Keypair,
    recent_blockhash: &Hash,
    lottery_result_rent: u64,
    lottery_id: u32,
    lottery_result: &Keypair,
    winner_wallet: &Pubkey,
    rewards_wallet: &Pubkey,
    slot_holders_wallet: &Pubkey,
    sollotto_labs_wallet: &Pubkey,
    staking_pool_wallet: &Keypair,
    lottery_authority: &Keypair,
) -> Result<(), TransportError> {
    let mut transaction = Transaction::new_with_payer(
        &[
            system_instruction::create_account(
                &payer.pubkey(),
                &lottery_result.pubkey(),
                lottery_result_rent,
                LotteryResultData::LEN as u64,
                &id(),
            ),
            sollotto_model_2::instruction::reward_winner(
                &id(),
                lottery_id,
                &lottery_result.pubkey(),
                winner_wallet,
                &rewards_wallet,
                &slot_holders_wallet,
                &sollotto_labs_wallet,
                &staking_pool_wallet.pubkey(),
                &lottery_authority.pubkey(),
            )
            .unwrap(),
        ],
        Some(&payer.pubkey()),
    );
    transaction.sign(
        &[
            payer,
            lottery_authority,
            staking_pool_wallet,
            lottery_result,
        ],
        *recent_blockhash,
    );
    banks_client.process_transaction(transaction).await?;
    Ok(())
}

async fn create_token_account(
    banks_client: &mut BanksClient,
    payer: &Keypair,
    recent_blockhash: &Hash,
    account: &Keypair,
    account_rent: u64,
    mint: &Pubkey,
    owner: &Pubkey,
) -> Result<(), TransportError> {
    let mut transaction = Transaction::new_with_payer(
        &[
            system_instruction::create_account(
                &payer.pubkey(),
                &account.pubkey(),
                account_rent,
                spl_token::state::Account::LEN as u64,
                &spl_token::id(),
            ),
            spl_token::instruction::initialize_account(
                &spl_token::id(),
                &account.pubkey(),
                mint,
                owner,
            )
            .unwrap(),
        ],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[payer, account], *recent_blockhash);
    banks_client.process_transaction(transaction).await?;
    Ok(())
}

async fn transfer_sol(
    banks_client: &mut BanksClient,
    recent_blockhash: &Hash,
    from: &Keypair,
    to: &Keypair,
    amount_sol: f64,
) -> Result<(), TransportError> {
    let mut transaction = system_transaction::transfer(
        from,
        &to.pubkey(),
        sol_to_lamports(amount_sol),
        *recent_blockhash,
    );
    transaction.sign(&[from], *recent_blockhash);
    banks_client.process_transaction(transaction).await?;
    Ok(())
}

async fn transfer_token(
    banks_client: &mut BanksClient,
    recent_blockhash: &Hash,
    payer: &Keypair,
    from: &Pubkey,
    to: &Pubkey,
    owner: &Keypair,
    amount: u64,
) -> Result<(), TransportError> {
    let mut transaction = Transaction::new_with_payer(
        &[spl_token::instruction::transfer(
            &spl_token::id(),
            from,
            to,
            &owner.pubkey(),
            &[],
            amount,
        )
        .unwrap()],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[payer, owner], *recent_blockhash);
    banks_client.process_transaction(transaction).await?;
    Ok(())
}

async fn get_token_balance(banks_client: &mut BanksClient, token_account: Pubkey) -> u64 {
    let account = banks_client
        .get_account(token_account)
        .await
        .unwrap()
        .unwrap();
    let account_data =
        spl_token::state::Account::unpack_from_slice(account.data.as_slice()).unwrap();
    account_data.amount
}

async fn check_token_balance(
    banks_client: &mut BanksClient,
    token_account: Pubkey,
    check_amount: f64,
) {
    assert_eq!(
        get_token_balance(banks_client, token_account).await,
        sol_to_lamports(check_amount)
    );
}

async fn check_balance(banks_client: &mut BanksClient, address: Pubkey, check_sol: f64) {
    assert_eq!(
        banks_client.get_balance(address).await.unwrap(),
        sol_to_lamports(check_sol)
    );
}

#[tokio::test]
async fn test_lottery() {
    let program = ProgramTest::new("sollotto", id(), processor!(Processor::process));
    let (mut banks_client, payer, recent_blockhash) = program.start().await;

    let rent = banks_client.get_rent().await.unwrap();
    let lottery_data_rent = rent.minimum_balance(LotteryData::LEN);
    let mint_rent = rent.minimum_balance(Mint::LEN);
    let token_account_rent = rent.minimum_balance(Account::LEN);
    let lottery_result_rent = rent.minimum_balance(LotteryResultData::LEN);

    let lottery_authority = Keypair::new();
    let staking_pool_wallet = Keypair::new();
    let staking_pool_token_mint = Keypair::new();
    let sollotto_staking_pool_token_account = Keypair::new();

    let rewards_wallet = Keypair::new();
    let slot_holders_rewards_wallet = Keypair::new();
    let sollotto_labs_wallet = Keypair::new();

    let number_of_users: usize = 2;
    let users_authority: Vec<Keypair> = (0..number_of_users).map(|_| Keypair::new()).collect();
    let users_staking_pool_token_account: Vec<Keypair> =
        (0..number_of_users).map(|_| Keypair::new()).collect();

    // Initialize lottery, mint, lottery associated token account
    initialize_lottery(
        &mut banks_client,
        &payer,
        &recent_blockhash,
        lottery_data_rent,
        mint_rent,
        token_account_rent,
        &staking_pool_wallet.pubkey(),
        &staking_pool_token_mint,
        &sollotto_staking_pool_token_account,
        &rewards_wallet.pubkey(),
        &slot_holders_rewards_wallet.pubkey(),
        &sollotto_labs_wallet.pubkey(),
        &lottery_authority,
    )
    .await
    .unwrap();

    // Create token associated account for users
    for i in 0..number_of_users {
        create_token_account(
            &mut banks_client,
            &payer,
            &recent_blockhash,
            &users_staking_pool_token_account[i],
            token_account_rent,
            &staking_pool_token_mint.pubkey(),
            &users_authority[i].pubkey(),
        )
        .await
        .unwrap();

        // Increase user balance
        transfer_sol(
            &mut banks_client,
            &recent_blockhash,
            &payer,
            &users_authority[i],
            10.0,
        )
        .await
        .unwrap();
    }

    // Users Deposit SOL
    for i in 0..number_of_users {
        deposit(
            &mut banks_client,
            &payer,
            &recent_blockhash,
            sol_to_lamports(1.0),
            &staking_pool_token_mint.pubkey(),
            &users_staking_pool_token_account[i].pubkey(),
            &staking_pool_wallet.pubkey(),
            &users_authority[i],
            &lottery_authority,
        )
        .await
        .unwrap();
    }

    // Check balances SOL and spl-token
    for i in 0..number_of_users {
        check_token_balance(
            &mut banks_client,
            users_staking_pool_token_account[i].pubkey(),
            1.0,
        )
        .await;
        check_balance(&mut banks_client, users_authority[i].pubkey(), 9.0).await;
    }
    check_balance(&mut banks_client, staking_pool_wallet.pubkey(), 2.0).await;

    // Increase staking pool wallet balance (get the prize pool)
    let prize_pool = 1.0;
    transfer_sol(
        &mut banks_client,
        &recent_blockhash,
        &payer,
        &staking_pool_wallet,
        prize_pool,
    )
    .await
    .unwrap();

    let lottery_id = 11223344;
    let lottery_result = Keypair::new();
    // Find and reward winner
    reward_winner(
        &mut banks_client,
        &payer,
        &recent_blockhash,
        lottery_result_rent,
        lottery_id,
        &lottery_result,
        &users_authority[0].pubkey(),
        &rewards_wallet.pubkey(),
        &slot_holders_rewards_wallet.pubkey(),
        &sollotto_labs_wallet.pubkey(),
        &staking_pool_wallet,
        &lottery_authority,
    )
    .await
    .unwrap();

    // Check balances (user[0] is winner)
    let winner_share = prize_pool * 0.95;
    check_balance(
        &mut banks_client,
        users_authority[0].pubkey(),
        9.0 + winner_share,
    )
    .await;
    check_balance(&mut banks_client, users_authority[1].pubkey(), 9.0).await;
    for i in 0..number_of_users {
        check_token_balance(
            &mut banks_client,
            users_staking_pool_token_account[i].pubkey(),
            1.0,
        )
        .await;
    }

    let sollotto_foundation_rewards_share = prize_pool * 0.04;
    check_balance(
        &mut banks_client,
        rewards_wallet.pubkey(),
        sollotto_foundation_rewards_share,
    )
    .await;
    let slot_holders_share = prize_pool * 0.006;
    check_balance(
        &mut banks_client,
        slot_holders_rewards_wallet.pubkey(),
        slot_holders_share,
    )
    .await;
    let sollotto_labs_share = prize_pool * 0.004;
    check_balance(
        &mut banks_client,
        sollotto_labs_wallet.pubkey(),
        sollotto_labs_share,
    )
    .await;

    check_balance(
        &mut banks_client,
        staking_pool_wallet.pubkey(),
        2.0,
    )
    .await;
}

#[tokio::test]
async fn test_insufficient_funds() {
    let program = ProgramTest::new("sollotto", id(), processor!(Processor::process));
    let (mut banks_client, payer, recent_blockhash) = program.start().await;

    let rent = banks_client.get_rent().await.unwrap();
    let lottery_data_rent = rent.minimum_balance(LotteryData::LEN);
    let mint_rent = rent.minimum_balance(Mint::LEN);
    let token_account_rent = rent.minimum_balance(Account::LEN);

    let lottery_authority = Keypair::new();
    let staking_pool_wallet = Keypair::new();
    let staking_pool_token_mint = Keypair::new();
    let sollotto_staking_pool_token_account = Keypair::new();

    let rewards_wallet = Keypair::new();
    let slot_holders_rewards_wallet = Keypair::new();
    let sollotto_labs_wallet = Keypair::new();

    let user_authority = Keypair::new();
    let user_staking_pool_token_account = Keypair::new();

    // Initialize lottery, mint, lottery associated token account
    initialize_lottery(
        &mut banks_client,
        &payer,
        &recent_blockhash,
        lottery_data_rent,
        mint_rent,
        token_account_rent,
        &staking_pool_wallet.pubkey(),
        &staking_pool_token_mint,
        &sollotto_staking_pool_token_account,
        &rewards_wallet.pubkey(),
        &slot_holders_rewards_wallet.pubkey(),
        &sollotto_labs_wallet.pubkey(),
        &lottery_authority,
    )
    .await
    .unwrap();

    // Create token associated account for user
    create_token_account(
        &mut banks_client,
        &payer,
        &recent_blockhash,
        &user_staking_pool_token_account,
        token_account_rent,
        &staking_pool_token_mint.pubkey(),
        &user_authority.pubkey(),
    )
    .await
    .unwrap();

    // User tries to deposit without SOL in balance
    assert_eq!(
        // SystemError::ResultWithNegativeLamports
        TransactionError::InstructionError(0, InstructionError::Custom(1)),
        deposit(
            &mut banks_client,
            &payer,
            &recent_blockhash,
            sol_to_lamports(0.5),
            &staking_pool_token_mint.pubkey(),
            &user_staking_pool_token_account.pubkey(),
            &staking_pool_wallet.pubkey(),
            &user_authority,
            &lottery_authority,
        )
        .await
        .unwrap_err()
        .unwrap()
    );

    // Send SOL to user
    transfer_sol(
        &mut banks_client,
        &recent_blockhash,
        &payer,
        &user_authority,
        1.0,
    )
    .await
    .unwrap();

    // User deposit SOL
    deposit(
        &mut banks_client,
        &payer,
        &recent_blockhash,
        sol_to_lamports(1.0),
        &staking_pool_token_mint.pubkey(),
        &user_staking_pool_token_account.pubkey(),
        &staking_pool_wallet.pubkey(),
        &user_authority,
        &lottery_authority,
    )
    .await
    .unwrap();

    // Check balances
    check_token_balance(
        &mut banks_client,
        user_staking_pool_token_account.pubkey(),
        1.0,
    )
    .await;
    check_balance(&mut banks_client, staking_pool_wallet.pubkey(), 1.0).await;
    check_balance(&mut banks_client, user_authority.pubkey(), 0.0).await;

    // User spent spl-token
    transfer_token(
        &mut banks_client,
        &recent_blockhash,
        &payer,
        &user_staking_pool_token_account.pubkey(),
        &sollotto_staking_pool_token_account.pubkey(),
        &user_authority,
        1_000_000_000, // 1.0 token
    )
    .await
    .unwrap();

    check_token_balance(
        &mut banks_client,
        user_staking_pool_token_account.pubkey(),
        0.0,
    )
    .await;

    // User tries to undeposit without spl-token in token balance
    assert_eq!(
        // spl token error insufficient funds
        TransactionError::InstructionError(0, InstructionError::Custom(1)),
        undeposit(
            &mut banks_client,
            &payer,
            &recent_blockhash,
            sol_to_lamports(1.0),
            &staking_pool_token_mint.pubkey(),
            &user_staking_pool_token_account.pubkey(),
            &staking_pool_wallet,
            &user_authority,
            &lottery_authority,
        )
        .await
        .unwrap_err()
        .unwrap()
    );

    transfer_token(
        &mut banks_client,
        &recent_blockhash,
        &payer,
        &sollotto_staking_pool_token_account.pubkey(),
        &user_staking_pool_token_account.pubkey(),
        &lottery_authority,
        1_000_000_000, // 1.0 token
    )
    .await
    .unwrap();

    // Lottery staking pool wallet spent SOL
    transfer_sol(
        &mut banks_client,
        &recent_blockhash,
        &staking_pool_wallet,
        &payer,
        0.5,
    )
    .await
    .unwrap();

    assert_eq!(
        // SystemError::ResultWithNegativeLamports
        TransactionError::InstructionError(0, InstructionError::Custom(1)),
        undeposit(
            &mut banks_client,
            &payer,
            &recent_blockhash,
            sol_to_lamports(0.8),
            &staking_pool_token_mint.pubkey(),
            &user_staking_pool_token_account.pubkey(),
            &staking_pool_wallet,
            &user_authority,
            &lottery_authority,
        )
        .await
        .unwrap_err()
        .unwrap()
    );
}
