use solana_program::{
    hash::Hash,
    instruction::InstructionError,
    native_token::sol_to_lamports,
    program_pack::Pack,
    system_instruction::{self},
};
use solana_program_test::*;
use solana_sdk::{signature::Keypair, transaction::TransactionError, transport::TransportError};
use sollotto_model_3::{error::LotteryError, id, processor::Processor, state::LotteryResultData};
use spl_token::{error::TokenError, ui_amount_to_amount};
use {
    solana_program::pubkey::Pubkey,
    solana_sdk::{signature::Signer, transaction::Transaction},
};

// Helper functions

async fn deposit(
    banks_client: &mut BanksClient,
    payer: &Keypair,
    recent_blockhash: &Hash,
    amount: u64,
    user_authority: &Keypair,
    staking_pool_authority: &Keypair,
    user_token_account: &Pubkey,
    user_staking_pool_account: &Pubkey,
    staking_pool_token_account: &Pubkey,
    staking_pool_token_mint: &Pubkey,
) -> Result<(), TransportError> {
    let mut transaction = Transaction::new_with_payer(
        &[sollotto_model_3::instruction::deposit(
            &id(),
            amount,
            &user_authority.pubkey(),
            &staking_pool_authority.pubkey(),
            user_token_account,
            user_staking_pool_account,
            staking_pool_token_account,
            staking_pool_token_mint,
        )
        .unwrap()],
        Some(&payer.pubkey()),
    );
    transaction.sign(
        &[payer, user_authority, staking_pool_authority],
        *recent_blockhash,
    );
    banks_client.process_transaction(transaction).await?;
    Ok(())
}

async fn unpool(
    banks_client: &mut BanksClient,
    payer: &Keypair,
    recent_blockhash: &Hash,
    amount: u64,
    user_authority: &Keypair,
    staking_pool_authority: &Keypair,
    user_token_account: &Pubkey,
    user_staking_pool_account: &Pubkey,
    staking_pool_token_account: &Pubkey,
    staking_pool_token_mint: &Pubkey,
) -> Result<(), TransportError> {
    let mut transaction = Transaction::new_with_payer(
        &[sollotto_model_3::instruction::unpool(
            &id(),
            amount,
            &user_authority.pubkey(),
            &staking_pool_authority.pubkey(),
            user_token_account,
            user_staking_pool_account,
            staking_pool_token_account,
            staking_pool_token_mint,
        )
        .unwrap()],
        Some(&payer.pubkey()),
    );
    transaction.sign(
        &[payer, user_authority, staking_pool_authority],
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
    random_number: u32,
    lottery_result: &Keypair,
    prize_pool_owner: &Keypair,
    prize_pool_token_account: &Pubkey,
    charity_token_account: &Pubkey,
    token_mint: &Pubkey,
    staking_pool_token_mint: &Pubkey,
    participants: &Vec<(Pubkey, Pubkey)>,
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
            sollotto_model_3::instruction::reward_winner(
                &id(),
                lottery_id,
                random_number,
                &prize_pool_owner.pubkey(),
                prize_pool_token_account,
                charity_token_account,
                token_mint,
                &lottery_result.pubkey(),
                staking_pool_token_mint,
                participants,
            )
            .unwrap(),
        ],
        Some(&payer.pubkey()),
    );
    transaction.sign(
        &[payer, prize_pool_owner, lottery_result],
        *recent_blockhash,
    );
    banks_client.process_transaction(transaction).await?;
    Ok(())
}

async fn create_token_mint(
    banks_client: &mut BanksClient,
    payer: &Keypair,
    recent_blockhash: &Hash,
    mint_rent: u64,
    decimals: u8,
    mint: &Keypair,
    mint_authority: &Pubkey,
) -> Result<(), TransportError> {
    let mut transaction = Transaction::new_with_payer(
        &[
            system_instruction::create_account(
                &payer.pubkey(),
                &mint.pubkey(),
                mint_rent,
                spl_token::state::Mint::LEN as u64,
                &spl_token::id(),
            ),
            spl_token::instruction::initialize_mint(
                &spl_token::id(),
                &mint.pubkey(),
                mint_authority,
                None,
                decimals,
            )
            .unwrap(),
        ],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[payer, mint], *recent_blockhash);
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

async fn mint_token(
    banks_client: &mut BanksClient,
    payer: &Keypair,
    recent_blockhash: &Hash,
    amount: u64,
    mint: &Pubkey,
    account: &Pubkey,
    owner: &Keypair,
) -> Result<(), TransportError> {
    let mut transaction = Transaction::new_with_payer(
        &[spl_token::instruction::mint_to(
            &spl_token::id(),
            mint,
            account,
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

#[tokio::test]
async fn test_lottery() {
    let program = ProgramTest::new("sollotto", id(), processor!(Processor::process));
    let (mut banks_client, payer, recent_blockhash) = program.start().await;

    let rent = banks_client.get_rent().await.unwrap();
    let lottery_result_rent = rent.minimum_balance(LotteryResultData::LEN);
    let account_rent = rent.minimum_balance(spl_token::state::Account::LEN);
    let mint_rent = rent.minimum_balance(spl_token::state::Mint::LEN);

    // Create Custom SPL Token
    let custom_token_mint = Keypair::new();
    let custom_mint_authority = Keypair::new();
    let decimals = 9;
    create_token_mint(
        &mut banks_client,
        &payer,
        &recent_blockhash,
        mint_rent,
        decimals,
        &custom_token_mint,
        &custom_mint_authority.pubkey(),
    )
    .await
    .unwrap();

    // Create Staking Pool Token
    let staking_pool_token_mint = Keypair::new();
    let staking_pool_custom_token_acc = Keypair::new();
    let staking_pool_owner = Keypair::new();
    create_token_mint(
        &mut banks_client,
        &payer,
        &recent_blockhash,
        mint_rent,
        decimals,
        &staking_pool_token_mint,
        &staking_pool_owner.pubkey(),
    )
    .await
    .unwrap();

    create_token_account(
        &mut banks_client,
        &payer,
        &recent_blockhash,
        &staking_pool_custom_token_acc,
        account_rent,
        &custom_token_mint.pubkey(),
        &staking_pool_owner.pubkey(),
    )
    .await
    .unwrap();

    // Initialize Custom SPL Token prize pool
    let token_prize_pool_owner = Keypair::new();
    let token_prize_pool_account = Keypair::new();
    create_token_account(
        &mut banks_client,
        &payer,
        &recent_blockhash,
        &token_prize_pool_account,
        account_rent,
        &custom_token_mint.pubkey(),
        &token_prize_pool_owner.pubkey(),
    )
    .await
    .unwrap();

    // Initialize lottery participants
    let number_of_users = 5;
    let users_initial_token_ui_amount = 5.0;
    let users_initial_token_amount = ui_amount_to_amount(users_initial_token_ui_amount, decimals);
    let users_wallets: Vec<Keypair> = (0..number_of_users).map(|_| Keypair::new()).collect();
    let users_token_accs: Vec<Keypair> = (0..number_of_users).map(|_| Keypair::new()).collect();
    let users_staking_pool_token_accs: Vec<Keypair> =
        (0..number_of_users).map(|_| Keypair::new()).collect();

    for i in 0..number_of_users {
        create_token_account(
            &mut banks_client,
            &payer,
            &recent_blockhash,
            &users_token_accs[i],
            account_rent,
            &custom_token_mint.pubkey(),
            &users_wallets[i].pubkey(),
        )
        .await
        .unwrap();

        create_token_account(
            &mut banks_client,
            &payer,
            &recent_blockhash,
            &users_staking_pool_token_accs[i],
            account_rent,
            &staking_pool_token_mint.pubkey(),
            &users_wallets[i].pubkey(),
        )
        .await
        .unwrap();
    }

    // BadCase: User deposit insufficiet funds
    assert_eq!(
        TransactionError::InstructionError(
            0,
            InstructionError::Custom(TokenError::InsufficientFunds as u32)
        ),
        deposit(
            &mut banks_client,
            &payer,
            &recent_blockhash,
            ui_amount_to_amount(2.0, decimals),
            &users_wallets[0],
            &staking_pool_owner,
            &users_token_accs[0].pubkey(),
            &users_staking_pool_token_accs[0].pubkey(),
            &staking_pool_custom_token_acc.pubkey(),
            &staking_pool_token_mint.pubkey(),
        )
        .await
        .unwrap_err()
        .unwrap()
    );

    for i in 0..number_of_users {
        mint_token(
            &mut banks_client,
            &payer,
            &recent_blockhash,
            users_initial_token_amount,
            &custom_token_mint.pubkey(),
            &users_token_accs[i].pubkey(),
            &custom_mint_authority,
        )
        .await
        .unwrap();
    }

    for user_account in &users_token_accs {
        check_token_balance(
            &mut banks_client,
            user_account.pubkey(),
            users_initial_token_ui_amount,
        )
        .await;
    }

    // Lottery #1. Users deposit 1 Token
    let user_deposit_ui_amount = 1.0;
    let user_deposit_amount = ui_amount_to_amount(user_deposit_ui_amount, decimals);
    for i in 0..number_of_users {
        deposit(
            &mut banks_client,
            &payer,
            &recent_blockhash,
            user_deposit_amount,
            &users_wallets[i],
            &staking_pool_owner,
            &users_token_accs[i].pubkey(),
            &users_staking_pool_token_accs[i].pubkey(),
            &staking_pool_custom_token_acc.pubkey(),
            &staking_pool_token_mint.pubkey(),
        )
        .await
        .unwrap();
    }

    // Check balances
    for user_account in &users_token_accs {
        check_token_balance(
            &mut banks_client,
            user_account.pubkey(),
            users_initial_token_ui_amount - user_deposit_ui_amount,
        )
        .await;
    }
    for user_staking_pool_account in &users_staking_pool_token_accs {
        check_token_balance(
            &mut banks_client,
            user_staking_pool_account.pubkey(),
            user_deposit_ui_amount,
        )
        .await;
    }
    check_token_balance(
        &mut banks_client,
        staking_pool_custom_token_acc.pubkey(),
        number_of_users as f64 * user_deposit_ui_amount,
    )
    .await;

    // Lottery #1. Initialize Charity
    let charity_token_account = Keypair::new();
    let charity_owner = Keypair::new();
    create_token_account(
        &mut banks_client,
        &payer,
        &recent_blockhash,
        &charity_token_account,
        account_rent,
        &custom_token_mint.pubkey(),
        &charity_owner.pubkey(),
    )
    .await
    .unwrap();

    // Lottery #1. Drawing and reward winner
    let lottery_id = 112233;
    let random_number = 3;
    let lottery_result = Keypair::new();
    let mut participants: Vec<(Pubkey, Pubkey)> = Vec::with_capacity(number_of_users);
    for i in 0..number_of_users {
        participants.push((
            users_token_accs[i].pubkey(),
            users_staking_pool_token_accs[i].pubkey(),
        ));
    }

    // BadCase: empty prize pool
    assert_eq!(
        TransactionError::InstructionError(
            1,
            InstructionError::Custom(LotteryError::EmptyPrizePool as u32)
        ),
        reward_winner(
            &mut banks_client,
            &payer,
            &recent_blockhash,
            lottery_result_rent,
            11223344,
            random_number,
            &lottery_result,
            &token_prize_pool_owner,
            &token_prize_pool_account.pubkey(),
            &charity_token_account.pubkey(),
            &custom_token_mint.pubkey(),
            &staking_pool_token_mint.pubkey(),
            &participants,
        )
        .await
        .unwrap_err()
        .unwrap()
    );

    // Filling the prize pool
    let prize_pool_ui_amount = 10.0;
    let prize_pool_amount = ui_amount_to_amount(prize_pool_ui_amount, decimals);
    mint_token(
        &mut banks_client,
        &payer,
        &recent_blockhash,
        prize_pool_amount,
        &custom_token_mint.pubkey(),
        &token_prize_pool_account.pubkey(),
        &custom_mint_authority,
    )
    .await
    .unwrap();

    reward_winner(
        &mut banks_client,
        &payer,
        &recent_blockhash,
        lottery_result_rent,
        lottery_id,
        random_number,
        &lottery_result,
        &token_prize_pool_owner,
        &token_prize_pool_account.pubkey(),
        &charity_token_account.pubkey(),
        &custom_token_mint.pubkey(),
        &staking_pool_token_mint.pubkey(),
        &participants,
    )
    .await
    .unwrap();

    // Check balances
    for i in 0..number_of_users {
        let mut check_amount = users_initial_token_ui_amount - user_deposit_ui_amount;
        if i as u32 == random_number {
            check_amount += prize_pool_ui_amount * 0.7;
        }

        check_token_balance(
            &mut banks_client,
            users_token_accs[i].pubkey(),
            check_amount,
        )
        .await;
    }
    check_token_balance(
        &mut banks_client,
        charity_token_account.pubkey(),
        prize_pool_ui_amount * 0.3,
    )
    .await;
    check_token_balance(&mut banks_client, token_prize_pool_account.pubkey(), 0.0).await;

    let lottery_result_account = banks_client
        .get_account(lottery_result.pubkey())
        .await
        .unwrap()
        .unwrap();
    let lottery_result_data =
        LotteryResultData::unpack_unchecked(&lottery_result_account.data).unwrap();
    assert_eq!(lottery_result_data.lottery_id, lottery_id);
    assert_eq!(
        lottery_result_data.winner,
        users_token_accs[random_number as usize].pubkey()
    );

    // User_3 unpool his tokens
    unpool(
        &mut banks_client,
        &payer,
        &recent_blockhash,
        user_deposit_amount,
        &users_wallets[3],
        &staking_pool_owner,
        &users_token_accs[3].pubkey(),
        &users_staking_pool_token_accs[3].pubkey(),
        &staking_pool_custom_token_acc.pubkey(),
        &staking_pool_token_mint.pubkey(),
    )
    .await
    .unwrap();

    // Check balances
    check_token_balance(
        &mut banks_client,
        users_staking_pool_token_accs[3].pubkey(),
        0.0,
    )
    .await;
    check_token_balance(
        &mut banks_client,
        users_token_accs[3].pubkey(),
        users_initial_token_ui_amount + prize_pool_ui_amount * 0.7,
    )
    .await;
    check_token_balance(
        &mut banks_client,
        staking_pool_custom_token_acc.pubkey(),
        (number_of_users - 1) as f64 * user_deposit_ui_amount,
    )
    .await;
}
