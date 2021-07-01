use solana_program::{
    hash::Hash, native_token::sol_to_lamports, program_pack::Pack, system_instruction,
};
use solana_program_test::*;
use solana_sdk::{signature::Keypair, system_transaction, transport::TransportError};
use sollotto::{
    processor::id,
    processor::Processor,
    state::{LotteryData, LotteryResultData, TicketData},
};
use {
    solana_program::pubkey::Pubkey,
    solana_sdk::{signature::Signer, transaction::Transaction},
};

// Helpers

async fn initialize_lottery(
    banks_client: &mut BanksClient,
    payer: &Keypair,
    recent_blockhash: &Hash,
    lottery_data_rent: u64,
    lottery_id: u32,
    charities: &Vec<Pubkey>,
    holding_wallet: &Pubkey,
    rewards_wallet: &Pubkey,
    slot_holders_rewards_wallet: &Pubkey,
    sollotto_labs_wallet: &Pubkey,
    lottery_authority: &Keypair,
) -> Result<(), TransportError> {
    assert_eq!(charities.len(), 4);
    let mut transaction = Transaction::new_with_payer(
        &[
            system_instruction::create_account(
                &payer.pubkey(),
                &lottery_authority.pubkey(),
                lottery_data_rent,
                sollotto::state::LotteryData::LEN as u64,
                &id(),
            ),
            sollotto::instruction::initialize_lottery(
                &id(),
                lottery_id,
                &charities[0],
                &charities[1],
                &charities[2],
                &charities[3],
                holding_wallet,
                rewards_wallet,
                slot_holders_rewards_wallet,
                sollotto_labs_wallet,
                &lottery_authority.pubkey(),
            )
            .unwrap(),
        ],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[payer, lottery_authority], *recent_blockhash);
    banks_client.process_transaction(transaction).await?;
    Ok(())
}

async fn purchase_ticket(
    banks_client: &mut BanksClient,
    payer: &Keypair,
    recent_blockhash: &Hash,
    ticket_data_rent: u64,
    charity: &Pubkey,
    ticket_number: &[u8; 6],
    holding_wallet: &Pubkey,
    ticket_authority: &Keypair,
    user_authority: &Keypair,
    lottery_authority: &Keypair,
) -> Result<(), TransportError> {
    let mut transaction = Transaction::new_with_payer(
        &[
            system_instruction::create_account(
                &payer.pubkey(),
                &ticket_authority.pubkey(),
                ticket_data_rent,
                sollotto::state::TicketData::LEN as u64,
                &id(),
            ),
            sollotto::instruction::purchase_ticket(
                &id(),
                charity,
                &user_authority.pubkey(),
                ticket_number,
                &ticket_authority.pubkey(),
                holding_wallet,
                &lottery_authority.pubkey(),
            )
            .unwrap(),
        ],
        Some(&payer.pubkey()),
    );
    transaction.sign(
        &[payer, ticket_authority, lottery_authority, user_authority],
        *recent_blockhash,
    );
    banks_client.process_transaction(transaction).await?;
    Ok(())
}

async fn store_winning_numbers(
    banks_client: &mut BanksClient,
    payer: &Keypair,
    recent_blockhash: &Hash,
    winning_numbers: &[u8; 6],
    lottery_authority: &Keypair,
) -> Result<(), TransportError> {
    let mut transaction = Transaction::new_with_payer(
        &[sollotto::instruction::store_winning_numbers(
            &id(),
            winning_numbers,
            &lottery_authority.pubkey(),
        )
        .unwrap()],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[payer, lottery_authority], *recent_blockhash);
    banks_client.process_transaction(transaction).await?;
    Ok(())
}

async fn reward_winners(
    banks_client: &mut BanksClient,
    payer: &Keypair,
    recent_blockhash: &Hash,
    lottery_result_data_rent: u64,
    rewards_wallet: &Pubkey,
    slot_holders_rewards_wallet: &Pubkey,
    sollotto_labs_wallet: &Pubkey,
    charities: &Vec<Pubkey>,
    participants: &Vec<(Pubkey, Pubkey)>,
    holding_wallet_authority: &Keypair,
    lottery_result_authority: &Keypair,
    lottery_authority: &Keypair,
) -> Result<(), TransportError> {
    assert_eq!(charities.len(), 4);
    let mut transaction = Transaction::new_with_payer(
        &[
            system_instruction::create_account(
                &payer.pubkey(),
                &lottery_result_authority.pubkey(),
                lottery_result_data_rent,
                sollotto::state::LotteryResultData::LEN as u64,
                &id(),
            ),
            sollotto::instruction::reward_winners(
                &id(),
                &lottery_authority.pubkey(),
                &lottery_result_authority.pubkey(),
                &holding_wallet_authority.pubkey(),
                rewards_wallet,
                slot_holders_rewards_wallet,
                sollotto_labs_wallet,
                &[charities[0], charities[1], charities[2], charities[3]],
                participants,
            )
            .unwrap(),
        ],
        Some(&payer.pubkey()),
    );
    transaction.sign(
        &[
            payer,
            lottery_result_authority,
            lottery_authority,
            holding_wallet_authority,
        ],
        *recent_blockhash,
    );
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

async fn check_balance(banks_client: &mut BanksClient, address: Pubkey, check_sol: f64) {
    assert_eq!(
        banks_client.get_balance(address).await.unwrap(),
        sol_to_lamports(check_sol)
    );
}

#[tokio::test]
async fn test_one_winner() {
    let program = ProgramTest::new("sollotto", id(), processor!(Processor::process));
    let (mut banks_client, payer, recent_blockhash) = program.start().await;

    let rent = banks_client.get_rent().await.unwrap();
    let lottery_result_data_rent = rent.minimum_balance(LotteryResultData::LEN);
    let ticket_data_rent = rent.minimum_balance(TicketData::LEN);
    let lottery_data_rent = rent.minimum_balance(LotteryData::LEN);

    let number_of_users = 5;
    let lottery_id = 112233;
    let lottery_authority = Keypair::new();
    let lottery_result = Keypair::new();
    let holding_wallet = Keypair::new();
    let rewards_wallet = Keypair::new();
    let slot_holders_rewards_wallet = Keypair::new();
    let sollotto_labs_wallet = Keypair::new();
    let charities: Vec<Keypair> = (0..4).map(|_| Keypair::new()).collect();
    let charities_pubkeys: Vec<Pubkey> = charities.iter().map(|c| c.pubkey()).collect();
    let users_wallets: Vec<Keypair> = (0..number_of_users).map(|_| Keypair::new()).collect();
    let users_wallets_pubkeys: Vec<Pubkey> = users_wallets.iter().map(|x| x.pubkey()).collect();
    let tickets: Vec<Keypair> = (0..number_of_users).map(|_| Keypair::new()).collect();
    let tickets_pubkeys: Vec<Pubkey> = tickets.iter().map(|x| x.pubkey()).collect();
    let winning_numbers = [1, 2, 3, 4, 5, 6];

    // Set initial balances for users
    for user in &users_wallets {
        transfer_sol(&mut banks_client, &recent_blockhash, &payer, user, 1.0)
            .await
            .unwrap();
    }

    initialize_lottery(
        &mut banks_client,
        &payer,
        &recent_blockhash,
        lottery_data_rent,
        lottery_id,
        &charities_pubkeys,
        &holding_wallet.pubkey(),
        &rewards_wallet.pubkey(),
        &slot_holders_rewards_wallet.pubkey(),
        &sollotto_labs_wallet.pubkey(),
        &lottery_authority,
    )
    .await
    .unwrap();

    // Users purchase tickets
    let mut ticket_numbers;
    for i in 0..number_of_users {
        if i == 0 {
            ticket_numbers = winning_numbers;
        } else {
            ticket_numbers = [1, 1, 1, 1, 1, 1];
        }
        purchase_ticket(
            &mut banks_client,
            &payer,
            &recent_blockhash,
            ticket_data_rent,
            &charities_pubkeys[0],
            &ticket_numbers,
            &holding_wallet.pubkey(),
            &tickets[i],
            &users_wallets[i],
            &lottery_authority,
        )
        .await
        .unwrap();
    }

    // Check balances
    let mut prize_pool_sol = 0.0;
    for user in &users_wallets {
        prize_pool_sol += 0.1;
        check_balance(&mut banks_client, user.pubkey(), 0.9).await;
    }

    check_balance(&mut banks_client, holding_wallet.pubkey(), prize_pool_sol).await;

    // Finaled lottery
    store_winning_numbers(
        &mut banks_client,
        &payer,
        &recent_blockhash,
        &winning_numbers,
        &lottery_authority,
    )
    .await
    .unwrap();

    let participants = (0..number_of_users)
        .map(|i| (tickets_pubkeys[i], users_wallets_pubkeys[i]))
        .collect();

    // Reward winners
    reward_winners(
        &mut banks_client,
        &payer,
        &recent_blockhash,
        lottery_result_data_rent,
        &rewards_wallet.pubkey(),
        &slot_holders_rewards_wallet.pubkey(),
        &sollotto_labs_wallet.pubkey(),
        &charities_pubkeys,
        &participants,
        &holding_wallet,
        &lottery_result,
        &lottery_authority,
    )
    .await
    .unwrap();

    // Check balances
    check_balance(&mut banks_client, holding_wallet.pubkey(), 0.0).await;
    check_balance(
        &mut banks_client,
        rewards_wallet.pubkey(),
        prize_pool_sol * 0.04,
    )
    .await;
    check_balance(
        &mut banks_client,
        slot_holders_rewards_wallet.pubkey(),
        prize_pool_sol * 0.006,
    )
    .await;
    check_balance(
        &mut banks_client,
        slot_holders_rewards_wallet.pubkey(),
        prize_pool_sol * 0.006,
    )
    .await;
    check_balance(
        &mut banks_client,
        sollotto_labs_wallet.pubkey(),
        prize_pool_sol * 0.004,
    )
    .await;

    // Charity balances
    check_balance(
        &mut banks_client,
        charities_pubkeys[0],
        prize_pool_sol * 0.3,
    )
    .await;
    for i in 1..4 {
        check_balance(&mut banks_client, charities_pubkeys[i], 0.0).await;
    }

    // Winner balance
    check_balance(
        &mut banks_client,
        users_wallets_pubkeys[0],
        prize_pool_sol * 0.65 + 0.9,
    )
    .await;
    // Loosers balances
    for i in 1..5 {
        check_balance(&mut banks_client, users_wallets_pubkeys[i], 0.9).await;
    }
}

#[tokio::test]
async fn test_without_winners() {
    let program = ProgramTest::new("sollotto", id(), processor!(Processor::process));
    let (mut banks_client, payer, recent_blockhash) = program.start().await;

    let rent = banks_client.get_rent().await.unwrap();
    let lottery_result_data_rent = rent.minimum_balance(LotteryResultData::LEN);
    let ticket_data_rent = rent.minimum_balance(TicketData::LEN);
    let lottery_data_rent = rent.minimum_balance(LotteryData::LEN);

    let number_of_users = 5;
    let lottery_id = 112233;
    let lottery_authority = Keypair::new();
    let lottery_result = Keypair::new();
    let holding_wallet = Keypair::new();
    let rewards_wallet = Keypair::new();
    let slot_holders_rewards_wallet = Keypair::new();
    let sollotto_labs_wallet = Keypair::new();
    let charities: Vec<Keypair> = (0..4).map(|_| Keypair::new()).collect();
    let charities_pubkeys: Vec<Pubkey> = charities.iter().map(|c| c.pubkey()).collect();
    let users_wallets: Vec<Keypair> = (0..number_of_users).map(|_| Keypair::new()).collect();
    let users_wallets_pubkeys: Vec<Pubkey> = users_wallets.iter().map(|x| x.pubkey()).collect();
    let tickets: Vec<Keypair> = (0..number_of_users).map(|_| Keypair::new()).collect();
    let tickets_pubkeys: Vec<Pubkey> = tickets.iter().map(|x| x.pubkey()).collect();
    let winning_numbers = [1, 2, 3, 4, 5, 6];

    // Set initial balances for users
    for user in &users_wallets {
        transfer_sol(&mut banks_client, &recent_blockhash, &payer, user, 1.0)
            .await
            .unwrap();
    }

    initialize_lottery(
        &mut banks_client,
        &payer,
        &recent_blockhash,
        lottery_data_rent,
        lottery_id,
        &charities_pubkeys,
        &holding_wallet.pubkey(),
        &rewards_wallet.pubkey(),
        &slot_holders_rewards_wallet.pubkey(),
        &sollotto_labs_wallet.pubkey(),
        &lottery_authority,
    )
    .await
    .unwrap();

    // Users purchase tickets
    let ticket_numbers = [1, 1, 1, 1, 1, 1];
    for i in 0..number_of_users {
        purchase_ticket(
            &mut banks_client,
            &payer,
            &recent_blockhash,
            ticket_data_rent,
            &charities_pubkeys[0],
            &ticket_numbers,
            &holding_wallet.pubkey(),
            &tickets[i],
            &users_wallets[i],
            &lottery_authority,
        )
        .await
        .unwrap();
    }

    // Check balances
    let mut prize_pool_sol = 0.0;
    for user in &users_wallets {
        prize_pool_sol += 0.1;
        check_balance(&mut banks_client, user.pubkey(), 0.9).await;
    }

    check_balance(&mut banks_client, holding_wallet.pubkey(), prize_pool_sol).await;

    // Finaled lottery
    store_winning_numbers(
        &mut banks_client,
        &payer,
        &recent_blockhash,
        &winning_numbers,
        &lottery_authority,
    )
    .await
    .unwrap();

    let participants = (0..number_of_users)
        .map(|i| (tickets_pubkeys[i], users_wallets_pubkeys[i]))
        .collect();

    // Reward winners
    reward_winners(
        &mut banks_client,
        &payer,
        &recent_blockhash,
        lottery_result_data_rent,
        &rewards_wallet.pubkey(),
        &slot_holders_rewards_wallet.pubkey(),
        &sollotto_labs_wallet.pubkey(),
        &charities_pubkeys,
        &participants,
        &holding_wallet,
        &lottery_result,
        &lottery_authority,
    )
    .await
    .unwrap();

    // Check balances
    check_balance(
        &mut banks_client,
        holding_wallet.pubkey(),
        prize_pool_sol * 0.65,
    )
    .await;
    check_balance(
        &mut banks_client,
        rewards_wallet.pubkey(),
        prize_pool_sol * 0.04,
    )
    .await;
    check_balance(
        &mut banks_client,
        slot_holders_rewards_wallet.pubkey(),
        prize_pool_sol * 0.006,
    )
    .await;
    check_balance(
        &mut banks_client,
        slot_holders_rewards_wallet.pubkey(),
        prize_pool_sol * 0.006,
    )
    .await;
    check_balance(
        &mut banks_client,
        sollotto_labs_wallet.pubkey(),
        prize_pool_sol * 0.004,
    )
    .await;

    // Charity balances
    check_balance(
        &mut banks_client,
        charities_pubkeys[0],
        prize_pool_sol * 0.3,
    )
    .await;
    for i in 1..4 {
        check_balance(&mut banks_client, charities_pubkeys[i], 0.0).await;
    }

    // Loosers balances
    for i in 0..5 {
        check_balance(&mut banks_client, users_wallets_pubkeys[i], 0.9).await;
    }
}

#[tokio::test]
async fn test_repeat_lottery() {
    // TODO: 5 users, 2 lottery one after another
}

#[tokio::test]
async fn test_charities_share() {
    let program = ProgramTest::new("sollotto", id(), processor!(Processor::process));
    let (mut banks_client, payer, recent_blockhash) = program.start().await;

    let rent = banks_client.get_rent().await.unwrap();
    let lottery_result_data_rent = rent.minimum_balance(LotteryResultData::LEN);
    let ticket_data_rent = rent.minimum_balance(TicketData::LEN);
    let lottery_data_rent = rent.minimum_balance(LotteryData::LEN);

    let number_of_users = 5;
    let lottery_id = 112233;
    let lottery_authority = Keypair::new();
    let lottery_result = Keypair::new();
    let holding_wallet = Keypair::new();
    let rewards_wallet = Keypair::new();
    let slot_holders_rewards_wallet = Keypair::new();
    let sollotto_labs_wallet = Keypair::new();
    let charities: Vec<Keypair> = (0..4).map(|_| Keypair::new()).collect();
    let charities_pubkeys: Vec<Pubkey> = charities.iter().map(|c| c.pubkey()).collect();
    let users_wallets: Vec<Keypair> = (0..number_of_users).map(|_| Keypair::new()).collect();
    let users_wallets_pubkeys: Vec<Pubkey> = users_wallets.iter().map(|x| x.pubkey()).collect();
    let tickets: Vec<Keypair> = (0..number_of_users).map(|_| Keypair::new()).collect();
    let tickets_pubkeys: Vec<Pubkey> = tickets.iter().map(|x| x.pubkey()).collect();
    let winning_numbers = [1, 2, 3, 4, 5, 6];

    // Set initial balances for users
    for user in &users_wallets {
        transfer_sol(&mut banks_client, &recent_blockhash, &payer, user, 1.0)
            .await
            .unwrap();
    }

    initialize_lottery(
        &mut banks_client,
        &payer,
        &recent_blockhash,
        lottery_data_rent,
        lottery_id,
        &charities_pubkeys,
        &holding_wallet.pubkey(),
        &rewards_wallet.pubkey(),
        &slot_holders_rewards_wallet.pubkey(),
        &sollotto_labs_wallet.pubkey(),
        &lottery_authority,
    )
    .await
    .unwrap();

    // Users purchase tickets
    let mut ticket_numbers;
    let mut user_charity;
    for i in 0..number_of_users {
        if i == 0 {
            ticket_numbers = winning_numbers;
        } else {
            ticket_numbers = [1, 1, 1, 1, 1, 1];
        }

        if i == 0 || i == 1 {
            user_charity = charities_pubkeys[0];
        } else if i == 2 || i == 3 {
            user_charity = charities_pubkeys[1];
        } else {
            user_charity = charities_pubkeys[2];
        }

        purchase_ticket(
            &mut banks_client,
            &payer,
            &recent_blockhash,
            ticket_data_rent,
            &user_charity,
            &ticket_numbers,
            &holding_wallet.pubkey(),
            &tickets[i],
            &users_wallets[i],
            &lottery_authority,
        )
        .await
        .unwrap();
    }

    // Check balances
    let mut prize_pool_sol = 0.0;
    for user in &users_wallets {
        prize_pool_sol += 0.1;
        check_balance(&mut banks_client, user.pubkey(), 0.9).await;
    }

    check_balance(&mut banks_client, holding_wallet.pubkey(), prize_pool_sol).await;

    // Finaled lottery
    store_winning_numbers(
        &mut banks_client,
        &payer,
        &recent_blockhash,
        &winning_numbers,
        &lottery_authority,
    )
    .await
    .unwrap();

    let participants = (0..number_of_users)
        .map(|i| (tickets_pubkeys[i], users_wallets_pubkeys[i]))
        .collect();

    // Reward winners
    reward_winners(
        &mut banks_client,
        &payer,
        &recent_blockhash,
        lottery_result_data_rent,
        &rewards_wallet.pubkey(),
        &slot_holders_rewards_wallet.pubkey(),
        &sollotto_labs_wallet.pubkey(),
        &charities_pubkeys,
        &participants,
        &holding_wallet,
        &lottery_result,
        &lottery_authority,
    )
    .await
    .unwrap();

    // Check balances
    check_balance(&mut banks_client, holding_wallet.pubkey(), 0.0).await;
    check_balance(
        &mut banks_client,
        rewards_wallet.pubkey(),
        prize_pool_sol * 0.04,
    )
    .await;
    check_balance(
        &mut banks_client,
        slot_holders_rewards_wallet.pubkey(),
        prize_pool_sol * 0.006,
    )
    .await;
    check_balance(
        &mut banks_client,
        slot_holders_rewards_wallet.pubkey(),
        prize_pool_sol * 0.006,
    )
    .await;
    check_balance(
        &mut banks_client,
        sollotto_labs_wallet.pubkey(),
        prize_pool_sol * 0.004,
    )
    .await;

    // Charity balances
    check_balance(
        &mut banks_client,
        charities_pubkeys[0],
        (prize_pool_sol * 0.3) / 2.0,
    )
    .await;
    check_balance(
        &mut banks_client,
        charities_pubkeys[1],
        (prize_pool_sol * 0.3) / 2.0,
    )
    .await;
    for i in 2..4 {
        check_balance(&mut banks_client, charities_pubkeys[i], 0.0).await;
    }

    // Winner balance
    check_balance(
        &mut banks_client,
        users_wallets_pubkeys[0],
        prize_pool_sol * 0.65 + 0.9,
    )
    .await;
    // Loosers balances
    for i in 1..5 {
        check_balance(&mut banks_client, users_wallets_pubkeys[i], 0.9).await;
    }

    // TODO: 5 users, 1 winner, 2 charity wins
}

#[tokio::test]
async fn test_many_users_one_winner() {
    // TODO: 25 users, 1 winner
}

#[tokio::test]
async fn test_many_users_many_winners() {
    // TODO: 25 users, 10 winners
}

#[tokio::test]
async fn test_many_users_without_winners() {
    // TODO: 25 users, 0 winners
}
