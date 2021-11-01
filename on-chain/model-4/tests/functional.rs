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
use sollotto_model_4::{error::LotteryError, id, processor::Processor, state::LotteryResultData};
use spl_token::ui_amount_to_amount;
use {
    solana_program::pubkey::Pubkey,
    solana_sdk::{signature::Signer, transaction::Transaction},
};

// Helper functions

async fn reward_winner(
    banks_client: &mut BanksClient,
    payer: &Keypair,
    recent_blockhash: &Hash,
    lottery_result_rent: u64,
    lottery_id: u32,
    random_number: u32,
    lottery_result: &Keypair,
    prize_pool_wallet: &Keypair,
    sollotto_rewards_wallet: &Pubkey,
    slot_holders_wallet: &Pubkey,
    sollotto_labs_wallet: &Pubkey,
    lifetime_ticket_token_owner: &Keypair,
    lifetime_ticket_token_mint: &Pubkey,
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
            sollotto_model_4::instruction::reward_winner(
                &id(),
                lottery_id,
                random_number,
                &prize_pool_wallet.pubkey(),
                sollotto_rewards_wallet,
                slot_holders_wallet,
                sollotto_labs_wallet,
                &lottery_result.pubkey(),
                &lifetime_ticket_token_owner.pubkey(),
                lifetime_ticket_token_mint,
                participants,
            )
            .unwrap(),
        ],
        Some(&payer.pubkey()),
    );
    transaction.sign(
        &[
            payer,
            lottery_result,
            prize_pool_wallet,
            lifetime_ticket_token_owner,
        ],
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

async fn mint_to(
    banks_client: &mut BanksClient,
    recent_blockhash: &Hash,
    payer: &Keypair,
    owner: &Keypair,
    mint: &Pubkey,
    account: &Pubkey,
    amount: u64,
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

async fn check_balance(banks_client: &mut BanksClient, account: &Keypair, check_amount: f64) {
    let balance = banks_client.get_balance(account.pubkey()).await.unwrap();
    assert_eq!(balance, sol_to_lamports(check_amount));
}

#[tokio::test]
async fn test_lottery() {
    let program = ProgramTest::new("sollotto", id(), processor!(Processor::process));
    let (mut banks_client, payer, recent_blockhash) = program.start().await;

    let rent = banks_client.get_rent().await.unwrap();
    let lottery_result_rent = rent.minimum_balance(LotteryResultData::LEN);
    let account_rent = rent.minimum_balance(spl_token::state::Account::LEN);
    let mint_rent = rent.minimum_balance(spl_token::state::Mint::LEN);

    // Set Up Mint for Lifetime Ticket token
    let lifetime_ticket_mint = Keypair::new();
    let lifetime_ticket_owner = Keypair::new();
    create_token_mint(
        &mut banks_client,
        &payer,
        &recent_blockhash,
        mint_rent,
        9,
        &lifetime_ticket_mint,
        &lifetime_ticket_owner.pubkey(),
    )
    .await
    .unwrap();

    let number_of_users = 5;
    let users_wallet: Vec<Keypair> = (0..number_of_users).map(|_| Keypair::new()).collect();
    let users_lifetime_account: Vec<Keypair> =
        (0..number_of_users).map(|_| Keypair::new()).collect();

    for i in 0..number_of_users {
        create_token_account(
            &mut banks_client,
            &payer,
            &recent_blockhash,
            &users_lifetime_account[i],
            account_rent,
            &lifetime_ticket_mint.pubkey(),
            &users_wallet[i].pubkey(),
        )
        .await
        .unwrap();
    }

    let participants = users_wallet
        .iter()
        .zip(&users_lifetime_account)
        .map(|(w, la)| (w.pubkey(), la.pubkey()))
        .collect();
    let prize_pool_wallet = Keypair::new();
    let lottery_result = Keypair::new();
    let sollotto_rewards_wallet = Keypair::new();
    let slot_holders_wallet = Keypair::new();
    let sollotto_labs_wallet = Keypair::new();
    let lottery_id = 11223344;
    let random_number = 3;

    // BadCase: Empty prize pool
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
            1,
            random_number,
            &lottery_result,
            &prize_pool_wallet,
            &sollotto_rewards_wallet.pubkey(),
            &slot_holders_wallet.pubkey(),
            &sollotto_labs_wallet.pubkey(),
            &lifetime_ticket_owner,
            &lifetime_ticket_mint.pubkey(),
            &participants,
        )
        .await
        .unwrap_err()
        .unwrap()
    );

    let prize_pool_amount = 35.0;
    transfer_sol(
        &mut banks_client,
        &recent_blockhash,
        &payer,
        &prize_pool_wallet,
        prize_pool_amount,
    )
    .await
    .unwrap();

    // BadCase: Users have not Lifetime tickets
    assert_eq!(
        TransactionError::InstructionError(
            1,
            InstructionError::Custom(LotteryError::InvalidParticipantsAccounts as u32)
        ),
        reward_winner(
            &mut banks_client,
            &payer,
            &recent_blockhash,
            lottery_result_rent,
            2,
            random_number,
            &lottery_result,
            &prize_pool_wallet,
            &sollotto_rewards_wallet.pubkey(),
            &slot_holders_wallet.pubkey(),
            &sollotto_labs_wallet.pubkey(),
            &lifetime_ticket_owner,
            &lifetime_ticket_mint.pubkey(),
            &participants,
        )
        .await
        .unwrap_err()
        .unwrap()
    );

    // Transfer Lifetime Ticket Tokens to users
    for account in &users_lifetime_account {
        mint_to(
            &mut banks_client,
            &recent_blockhash,
            &payer,
            &lifetime_ticket_owner,
            &lifetime_ticket_mint.pubkey(),
            &account.pubkey(),
            ui_amount_to_amount(1.0, 9),
        )
        .await
        .unwrap();
    }

    // Reward winner
    reward_winner(
        &mut banks_client,
        &payer,
        &recent_blockhash,
        lottery_result_rent,
        lottery_id,
        random_number,
        &lottery_result,
        &prize_pool_wallet,
        &sollotto_rewards_wallet.pubkey(),
        &slot_holders_wallet.pubkey(),
        &sollotto_labs_wallet.pubkey(),
        &lifetime_ticket_owner,
        &lifetime_ticket_mint.pubkey(),
        &participants,
    )
    .await
    .unwrap();

    // Check balances
    check_balance(&mut banks_client, &prize_pool_wallet, 0.0).await;
    check_balance(
        &mut banks_client,
        &sollotto_rewards_wallet,
        prize_pool_amount * 0.04,
    )
    .await;
    check_balance(
        &mut banks_client,
        &slot_holders_wallet,
        prize_pool_amount * 0.006,
    )
    .await;
    check_balance(
        &mut banks_client,
        &sollotto_labs_wallet,
        prize_pool_amount * 0.004,
    )
    .await;
    for i in 0..number_of_users {
        let mut check_amout = 0.0;
        if i == random_number as usize {
            check_amout = prize_pool_amount * 0.95;
        }

        check_balance(&mut banks_client, &users_wallet[i], check_amout).await;
    }

    // Check Result account
    let lottery_result_acc = banks_client
        .get_account(lottery_result.pubkey())
        .await
        .unwrap()
        .unwrap();
    let lottery_result_data =
        LotteryResultData::unpack_unchecked(&lottery_result_acc.data).unwrap();
    assert_eq!(lottery_result_data.lottery_id, lottery_id);
    assert_eq!(
        lottery_result_data.winner,
        users_wallet[random_number as usize].pubkey()
    );
}
