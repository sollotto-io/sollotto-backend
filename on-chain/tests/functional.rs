// use solana_program::{
//     hash::Hash,
//     instruction::InstructionError,
//     native_token::{lamports_to_sol, sol_to_lamports},
//     program_pack::Pack,
//     system_instruction,
// };
// use solana_program_test::*;
// use solana_sdk::{
//     signature::Keypair, system_transaction, transaction::TransactionError,
//     transport::TransportError,
// };
// use sollotto::{
//     processor::id,
//     processor::Processor,
//     state::{LotteryData, LotteryResultData, TicketData},
// };
// use {
//     solana_program::pubkey::Pubkey,
//     solana_sdk::{signature::Signer, transaction::Transaction},
// };

// // Helpers

// async fn initialize_lottery(
//     banks_client: &mut BanksClient,
//     payer: &Keypair,
//     recent_blockhash: &Hash,
//     lottery_data_rent: u64,
//     lottery_id: u32,
//     charities: &Vec<Pubkey>,
//     holding_wallet: &Pubkey,
//     rewards_wallet: &Pubkey,
//     slot_holders_rewards_wallet: &Pubkey,
//     sollotto_labs_wallet: &Pubkey,
//     randomness_account: &Pubkey,
//     lottery_authority: &Keypair,
// ) -> Result<(), TransportError> {
//     assert_eq!(charities.len(), 4);
//     let mut transaction = Transaction::new_with_payer(
//         &[
//             system_instruction::create_account(
//                 &payer.pubkey(),
//                 &lottery_authority.pubkey(),
//                 lottery_data_rent,
//                 sollotto::state::LotteryData::LEN as u64,
//                 &id(),
//             ),
//             sollotto::instruction::initialize_lottery(
//                 &id(),
//                 lottery_id,
//                 &charities[0],
//                 &charities[1],
//                 &charities[2],
//                 &charities[3],
//                 holding_wallet,
//                 rewards_wallet,
//                 slot_holders_rewards_wallet,
//                 sollotto_labs_wallet,
//                 randomness_account,
//                 &lottery_authority.pubkey(),
//             )
//             .unwrap(),
//         ],
//         Some(&payer.pubkey()),
//     );
//     transaction.sign(&[payer, lottery_authority], *recent_blockhash);
//     banks_client.process_transaction(transaction).await?;
//     Ok(())
// }

// async fn initialize_lottery_without_creation(
//     banks_client: &mut BanksClient,
//     payer: &Keypair,
//     recent_blockhash: &Hash,
//     lottery_id: u32,
//     charities: &Vec<Pubkey>,
//     holding_wallet: &Pubkey,
//     rewards_wallet: &Pubkey,
//     slot_holders_rewards_wallet: &Pubkey,
//     sollotto_labs_wallet: &Pubkey,
//     randomness_account: &Pubkey,
//     lottery_authority: &Keypair,
// ) -> Result<(), TransportError> {
//     assert_eq!(charities.len(), 4);
//     let mut transaction = Transaction::new_with_payer(
//         &[sollotto::instruction::initialize_lottery(
//             &id(),
//             lottery_id,
//             &charities[0],
//             &charities[1],
//             &charities[2],
//             &charities[3],
//             holding_wallet,
//             rewards_wallet,
//             slot_holders_rewards_wallet,
//             sollotto_labs_wallet,
//             randomness_account,
//             &lottery_authority.pubkey(),
//         )
//         .unwrap()],
//         Some(&payer.pubkey()),
//     );
//     transaction.sign(&[payer, lottery_authority], *recent_blockhash);
//     banks_client.process_transaction(transaction).await?;
//     Ok(())
// }

// async fn purchase_ticket(
//     banks_client: &mut BanksClient,
//     payer: &Keypair,
//     recent_blockhash: &Hash,
//     ticket_data_rent: u64,
//     charity: &Pubkey,
//     ticket_number: &[u8; 6],
//     holding_wallet: &Pubkey,
//     ticket_authority: &Keypair,
//     user_authority: &Keypair,
//     lottery_authority: &Keypair,
// ) -> Result<(), TransportError> {
//     let mut transaction = Transaction::new_with_payer(
//         &[
//             system_instruction::create_account(
//                 &payer.pubkey(),
//                 &ticket_authority.pubkey(),
//                 ticket_data_rent,
//                 sollotto::state::TicketData::LEN as u64,
//                 &id(),
//             ),
//             sollotto::instruction::purchase_ticket(
//                 &id(),
//                 charity,
//                 &user_authority.pubkey(),
//                 ticket_number,
//                 &ticket_authority.pubkey(),
//                 holding_wallet,
//                 &lottery_authority.pubkey(),
//             )
//             .unwrap(),
//         ],
//         Some(&payer.pubkey()),
//     );
//     transaction.sign(
//         &[payer, ticket_authority, lottery_authority, user_authority],
//         *recent_blockhash,
//     );
//     banks_client.process_transaction(transaction).await?;
//     Ok(())
// }

// async fn create_ticket_account(
//     banks_client: &mut BanksClient,
//     payer: &Keypair,
//     recent_blockhash: &Hash,
//     ticket_data_rent: u64,
//     ticket_authority: &Keypair,
// ) -> Result<(), TransportError> {
//     let mut transaction = Transaction::new_with_payer(
//         &[system_instruction::create_account(
//             &payer.pubkey(),
//             &ticket_authority.pubkey(),
//             ticket_data_rent,
//             sollotto::state::TicketData::LEN as u64,
//             &id(),
//         )],
//         Some(&payer.pubkey()),
//     );
//     transaction.sign(&[payer, ticket_authority], *recent_blockhash);
//     banks_client.process_transaction(transaction).await?;
//     Ok(())
// }

// async fn purchase_ticket_without_creation(
//     banks_client: &mut BanksClient,
//     payer: &Keypair,
//     recent_blockhash: &Hash,
//     charity: &Pubkey,
//     ticket_number: &[u8; 6],
//     holding_wallet: &Pubkey,
//     ticket_authority: &Keypair,
//     user_authority: &Keypair,
//     lottery_authority: &Keypair,
// ) -> Result<(), TransportError> {
//     let mut transaction = Transaction::new_with_payer(
//         &[sollotto::instruction::purchase_ticket(
//             &id(),
//             charity,
//             &user_authority.pubkey(),
//             ticket_number,
//             &ticket_authority.pubkey(),
//             holding_wallet,
//             &lottery_authority.pubkey(),
//         )
//         .unwrap()],
//         Some(&payer.pubkey()),
//     );
//     transaction.sign(
//         &[payer, lottery_authority, user_authority],
//         *recent_blockhash,
//     );
//     banks_client.process_transaction(transaction).await?;
//     Ok(())
// }

// async fn store_winning_numbers(
//     banks_client: &mut BanksClient,
//     payer: &Keypair,
//     recent_blockhash: &Hash,
//     winning_numbers: &[u8; 6],
//     lottery_authority: &Keypair,
// ) -> Result<(), TransportError> {
//     let mut transaction = Transaction::new_with_payer(
//         &[sollotto::instruction::store_winning_numbers(
//             &id(),
//             winning_numbers,
//             &lottery_authority.pubkey(),
//         )
//         .unwrap()],
//         Some(&payer.pubkey()),
//     );
//     transaction.sign(&[payer, lottery_authority], *recent_blockhash);
//     banks_client.process_transaction(transaction).await?;
//     Ok(())
// }

// async fn reward_winners(
//     banks_client: &mut BanksClient,
//     payer: &Keypair,
//     recent_blockhash: &Hash,
//     lottery_result_data_rent: u64,
//     rewards_wallet: &Pubkey,
//     slot_holders_rewards_wallet: &Pubkey,
//     sollotto_labs_wallet: &Pubkey,
//     charities: &Vec<Pubkey>,
//     participants: &Vec<(Pubkey, Pubkey)>,
//     holding_wallet_authority: &Keypair,
//     lottery_result_authority: &Keypair,
//     lottery_authority: &Keypair,
// ) -> Result<(), TransportError> {
//     assert_eq!(charities.len(), 4);
//     let mut transaction = Transaction::new_with_payer(
//         &[
//             system_instruction::create_account(
//                 &payer.pubkey(),
//                 &lottery_result_authority.pubkey(),
//                 lottery_result_data_rent,
//                 sollotto::state::LotteryResultData::LEN as u64,
//                 &id(),
//             ),
//             sollotto::instruction::reward_winners(
//                 &id(),
//                 &lottery_authority.pubkey(),
//                 &lottery_result_authority.pubkey(),
//                 &holding_wallet_authority.pubkey(),
//                 rewards_wallet,
//                 slot_holders_rewards_wallet,
//                 sollotto_labs_wallet,
//                 &[charities[0], charities[1], charities[2], charities[3]],
//                 participants,
//             )
//             .unwrap(),
//         ],
//         Some(&payer.pubkey()),
//     );
//     transaction.sign(
//         &[
//             payer,
//             lottery_result_authority,
//             lottery_authority,
//             holding_wallet_authority,
//         ],
//         *recent_blockhash,
//     );
//     banks_client.process_transaction(transaction).await?;
//     Ok(())
// }

// async fn update_charity(
//     banks_client: &mut BanksClient,
//     payer: &Keypair,
//     recent_blockhash: &Hash,
//     charities: &Vec<Pubkey>,
//     lottery_authority: &Keypair,
// ) -> Result<(), TransportError> {
//     assert_eq!(charities.len(), 4);
//     let mut transaction = Transaction::new_with_payer(
//         &[sollotto::instruction::update_charity(
//             &id(),
//             &charities[0],
//             &charities[1],
//             &charities[2],
//             &charities[3],
//             &lottery_authority.pubkey(),
//         )
//         .unwrap()],
//         Some(&payer.pubkey()),
//     );
//     transaction.sign(&[payer, lottery_authority], *recent_blockhash);
//     banks_client.process_transaction(transaction).await?;
//     Ok(())
// }

// async fn update_sollotto_wallets(
//     banks_client: &mut BanksClient,
//     payer: &Keypair,
//     recent_blockhash: &Hash,
//     holding_wallet: &Pubkey,
//     rewards_wallet: &Pubkey,
//     slot_holders_rewards_wallet: &Pubkey,
//     sollotto_labs_wallet: &Pubkey,
//     lottery_authority: &Keypair,
// ) -> Result<(), TransportError> {
//     let mut transaction = Transaction::new_with_payer(
//         &[sollotto::instruction::update_sollotto_wallets(
//             &id(),
//             holding_wallet,
//             rewards_wallet,
//             slot_holders_rewards_wallet,
//             sollotto_labs_wallet,
//             &lottery_authority.pubkey(),
//         )
//         .unwrap()],
//         Some(&payer.pubkey()),
//     );
//     transaction.sign(&[payer, lottery_authority], *recent_blockhash);
//     banks_client.process_transaction(transaction).await?;
//     Ok(())
// }

// async fn transfer_sol(
//     banks_client: &mut BanksClient,
//     recent_blockhash: &Hash,
//     from: &Keypair,
//     to: &Keypair,
//     amount_sol: f64,
// ) -> Result<(), TransportError> {
//     let mut transaction = system_transaction::transfer(
//         from,
//         &to.pubkey(),
//         sol_to_lamports(amount_sol),
//         *recent_blockhash,
//     );
//     transaction.sign(&[from], *recent_blockhash);
//     banks_client.process_transaction(transaction).await?;
//     Ok(())
// }

// async fn check_balance(banks_client: &mut BanksClient, address: Pubkey, check_sol: f64) {
//     assert_eq!(
//         banks_client.get_balance(address).await.unwrap(),
//         sol_to_lamports(check_sol)
//     );
// }

// #[tokio::test]
// async fn test_one_winner() {
//     let program = ProgramTest::new("sollotto", id(), processor!(Processor::process));
//     let (mut banks_client, payer, recent_blockhash) = program.start().await;

//     let rent = banks_client.get_rent().await.unwrap();
//     let lottery_result_data_rent = rent.minimum_balance(LotteryResultData::LEN);
//     let ticket_data_rent = rent.minimum_balance(TicketData::LEN);
//     let lottery_data_rent = rent.minimum_balance(LotteryData::LEN);

//     let number_of_users = 5;
//     let lottery_id = 112233;
//     let lottery_authority = Keypair::new();
//     let lottery_result = Keypair::new();
//     let holding_wallet = Keypair::new();
//     let rewards_wallet = Keypair::new();
//     let slot_holders_rewards_wallet = Keypair::new();
//     let sollotto_labs_wallet = Keypair::new();
//     let randomness_account = Keypair::new();
//     let charities: Vec<Keypair> = (0..4).map(|_| Keypair::new()).collect();
//     let charities_pubkeys: Vec<Pubkey> = charities.iter().map(|c| c.pubkey()).collect();
//     let users_wallets: Vec<Keypair> = (0..number_of_users).map(|_| Keypair::new()).collect();
//     let users_wallets_pubkeys: Vec<Pubkey> = users_wallets.iter().map(|x| x.pubkey()).collect();
//     let tickets: Vec<Keypair> = (0..number_of_users).map(|_| Keypair::new()).collect();
//     let tickets_pubkeys: Vec<Pubkey> = tickets.iter().map(|x| x.pubkey()).collect();
//     let winning_numbers = [1, 2, 3, 4, 5, 6];

//     // Set initial balances for users
//     for user in &users_wallets {
//         transfer_sol(&mut banks_client, &recent_blockhash, &payer, user, 1.0)
//             .await
//             .unwrap();
//     }

//     initialize_lottery(
//         &mut banks_client,
//         &payer,
//         &recent_blockhash,
//         lottery_data_rent,
//         lottery_id,
//         &charities_pubkeys,
//         &holding_wallet.pubkey(),
//         &rewards_wallet.pubkey(),
//         &slot_holders_rewards_wallet.pubkey(),
//         &sollotto_labs_wallet.pubkey(),
//         &randomness_account.pubkey(),
//         &lottery_authority,
//     )
//     .await
//     .unwrap();

//     // Users purchase tickets
//     let mut ticket_numbers;
//     for i in 0..number_of_users {
//         if i == 0 {
//             ticket_numbers = winning_numbers;
//         } else {
//             ticket_numbers = [1, 1, 1, 1, 1, 1];
//         }
//         purchase_ticket(
//             &mut banks_client,
//             &payer,
//             &recent_blockhash,
//             ticket_data_rent,
//             &charities_pubkeys[0],
//             &ticket_numbers,
//             &holding_wallet.pubkey(),
//             &tickets[i],
//             &users_wallets[i],
//             &lottery_authority,
//         )
//         .await
//         .unwrap();
//     }

//     // Check balances
//     let mut prize_pool_sol = 0.0;
//     for user in &users_wallets {
//         prize_pool_sol += 0.1;
//         check_balance(&mut banks_client, user.pubkey(), 0.9).await;
//     }

//     check_balance(&mut banks_client, holding_wallet.pubkey(), prize_pool_sol).await;

//     // Finaled lottery
//     store_winning_numbers(
//         &mut banks_client,
//         &payer,
//         &recent_blockhash,
//         &winning_numbers,
//         &lottery_authority,
//     )
//     .await
//     .unwrap();

//     let participants = (0..number_of_users)
//         .map(|i| (tickets_pubkeys[i], users_wallets_pubkeys[i]))
//         .collect();

//     // Reward winners
//     reward_winners(
//         &mut banks_client,
//         &payer,
//         &recent_blockhash,
//         lottery_result_data_rent,
//         &rewards_wallet.pubkey(),
//         &slot_holders_rewards_wallet.pubkey(),
//         &sollotto_labs_wallet.pubkey(),
//         &charities_pubkeys,
//         &participants,
//         &holding_wallet,
//         &lottery_result,
//         &lottery_authority,
//     )
//     .await
//     .unwrap();

//     // Check balances
//     check_balance(&mut banks_client, holding_wallet.pubkey(), 0.0).await;
//     check_balance(
//         &mut banks_client,
//         rewards_wallet.pubkey(),
//         prize_pool_sol * 0.04,
//     )
//     .await;
//     check_balance(
//         &mut banks_client,
//         slot_holders_rewards_wallet.pubkey(),
//         prize_pool_sol * 0.006,
//     )
//     .await;
//     check_balance(
//         &mut banks_client,
//         sollotto_labs_wallet.pubkey(),
//         prize_pool_sol * 0.004,
//     )
//     .await;

//     // Charity balances
//     check_balance(
//         &mut banks_client,
//         charities_pubkeys[0],
//         prize_pool_sol * 0.3,
//     )
//     .await;
//     for i in 1..4 {
//         check_balance(&mut banks_client, charities_pubkeys[i], 0.0).await;
//     }

//     // Winner balance
//     check_balance(
//         &mut banks_client,
//         users_wallets_pubkeys[0],
//         prize_pool_sol * 0.65 + 0.9,
//     )
//     .await;
//     // Loosers balances
//     for i in 1..5 {
//         check_balance(&mut banks_client, users_wallets_pubkeys[i], 0.9).await;
//     }
// }

// #[tokio::test]
// async fn test_update_wallets() {
//     let program = ProgramTest::new("sollotto", id(), processor!(Processor::process));
//     let (mut banks_client, payer, recent_blockhash) = program.start().await;

//     let rent = banks_client.get_rent().await.unwrap();
//     let lottery_result_data_rent = rent.minimum_balance(LotteryResultData::LEN);
//     let ticket_data_rent = rent.minimum_balance(TicketData::LEN);
//     let lottery_data_rent = rent.minimum_balance(LotteryData::LEN);

//     let number_of_users = 5;
//     let lottery_id = 112233;
//     let lottery_authority = Keypair::new();
//     let lottery_result = Keypair::new();
//     let holding_wallet = Keypair::new();
//     let rewards_wallet = Keypair::new();
//     let slot_holders_rewards_wallet = Keypair::new();
//     let sollotto_labs_wallet = Keypair::new();
//     let randomness_account = Keypair::new();
//     let charities: Vec<Keypair> = (0..4).map(|_| Keypair::new()).collect();
//     let charities_pubkeys: Vec<Pubkey> = charities.iter().map(|c| c.pubkey()).collect();
//     let users_wallets: Vec<Keypair> = (0..number_of_users).map(|_| Keypair::new()).collect();
//     let users_wallets_pubkeys: Vec<Pubkey> = users_wallets.iter().map(|x| x.pubkey()).collect();
//     let tickets: Vec<Keypair> = (0..number_of_users).map(|_| Keypair::new()).collect();
//     let tickets_pubkeys: Vec<Pubkey> = tickets.iter().map(|x| x.pubkey()).collect();
//     let winning_numbers = [1, 2, 3, 4, 5, 6];

//     // Set initial balances for users
//     for user in &users_wallets {
//         transfer_sol(&mut banks_client, &recent_blockhash, &payer, user, 1.0)
//             .await
//             .unwrap();
//     }

//     initialize_lottery(
//         &mut banks_client,
//         &payer,
//         &recent_blockhash,
//         lottery_data_rent,
//         lottery_id,
//         &charities_pubkeys,
//         &holding_wallet.pubkey(),
//         &rewards_wallet.pubkey(),
//         &slot_holders_rewards_wallet.pubkey(),
//         &sollotto_labs_wallet.pubkey(),
//         &randomness_account.pubkey(),
//         &lottery_authority,
//     )
//     .await
//     .unwrap();

//     // Users purchase tickets
//     for i in 0..number_of_users {
//         purchase_ticket(
//             &mut banks_client,
//             &payer,
//             &recent_blockhash,
//             ticket_data_rent,
//             &charities_pubkeys[0],
//             &[3, 3, 3, 3, 3, 3],
//             &holding_wallet.pubkey(),
//             &tickets[i],
//             &users_wallets[i],
//             &lottery_authority,
//         )
//         .await
//         .unwrap();
//     }

//     // Check balances
//     let mut prize_pool_sol = 0.0;
//     for user in &users_wallets {
//         prize_pool_sol += 0.1;
//         check_balance(&mut banks_client, user.pubkey(), 0.9).await;
//     }

//     check_balance(&mut banks_client, holding_wallet.pubkey(), prize_pool_sol).await;

//     // Finaled lottery
//     store_winning_numbers(
//         &mut banks_client,
//         &payer,
//         &recent_blockhash,
//         &winning_numbers,
//         &lottery_authority,
//     )
//     .await
//     .unwrap();

//     let participants = (0..number_of_users)
//         .map(|i| (tickets_pubkeys[i], users_wallets_pubkeys[i]))
//         .collect();

//     // Update charities
//     let new_charities: Vec<Keypair> = (0..4).map(|_| Keypair::new()).collect();
//     let new_charities_pubkeys: Vec<Pubkey> = new_charities.iter().map(|c| c.pubkey()).collect();
//     update_charity(
//         &mut banks_client,
//         &payer,
//         &recent_blockhash,
//         &new_charities_pubkeys,
//         &lottery_authority,
//     )
//     .await
//     .unwrap();

//     // Reward winners
//     reward_winners(
//         &mut banks_client,
//         &payer,
//         &recent_blockhash,
//         lottery_result_data_rent,
//         &rewards_wallet.pubkey(),
//         &slot_holders_rewards_wallet.pubkey(),
//         &sollotto_labs_wallet.pubkey(),
//         &new_charities_pubkeys,
//         &participants,
//         &holding_wallet,
//         &lottery_result,
//         &lottery_authority,
//     )
//     .await
//     .unwrap();

//     // Check balances
//     let prize_pool_1_remain = prize_pool_sol * 0.65;
//     check_balance(
//         &mut banks_client,
//         holding_wallet.pubkey(),
//         prize_pool_1_remain,
//     )
//     .await;
//     check_balance(
//         &mut banks_client,
//         rewards_wallet.pubkey(),
//         prize_pool_sol * 0.04,
//     )
//     .await;
//     check_balance(
//         &mut banks_client,
//         slot_holders_rewards_wallet.pubkey(),
//         prize_pool_sol * 0.006,
//     )
//     .await;
//     check_balance(
//         &mut banks_client,
//         sollotto_labs_wallet.pubkey(),
//         prize_pool_sol * 0.004,
//     )
//     .await;

//     // Charity balances
//     for i in 0..4 {
//         check_balance(&mut banks_client, charities_pubkeys[i], 0.0).await;
//     }

//     // New charity get the reward
//     check_balance(
//         &mut banks_client,
//         new_charities_pubkeys[0],
//         prize_pool_sol * 0.3,
//     )
//     .await;
//     for i in 1..4 {
//         check_balance(&mut banks_client, new_charities_pubkeys[i], 0.0).await;
//     }

//     // Update sollotto wallets and repeat lottery
//     let lottery_id = 222222;
//     initialize_lottery_without_creation(
//         &mut banks_client,
//         &payer,
//         &recent_blockhash,
//         lottery_id,
//         &charities_pubkeys,
//         &holding_wallet.pubkey(),
//         &rewards_wallet.pubkey(),
//         &slot_holders_rewards_wallet.pubkey(),
//         &sollotto_labs_wallet.pubkey(),
        
//         &lottery_authority,
//     )
//     .await
//     .unwrap();

//     let new_rewards_wallet = Keypair::new();
//     let new_slot_holders_rewards_wallet = Keypair::new();
//     let new_sollotto_labs_wallet = Keypair::new();
//     update_sollotto_wallets(
//         &mut banks_client,
//         &payer,
//         &recent_blockhash,
//         &holding_wallet.pubkey(),
//         &new_rewards_wallet.pubkey(),
//         &new_slot_holders_rewards_wallet.pubkey(),
//         &new_sollotto_labs_wallet.pubkey(),
//         &lottery_authority,
//     )
//     .await
//     .unwrap();

//     // Users purchase tickets
//     let tickets: Vec<Keypair> = (0..number_of_users).map(|_| Keypair::new()).collect();
//     let tickets_pubkeys: Vec<Pubkey> = tickets.iter().map(|x| x.pubkey()).collect();
//     for i in 0..number_of_users {
//         purchase_ticket(
//             &mut banks_client,
//             &payer,
//             &recent_blockhash,
//             ticket_data_rent,
//             &charities_pubkeys[0],
//             &[4, 4, 4, 4, 4, 4],
//             &holding_wallet.pubkey(),
//             &tickets[i],
//             &users_wallets[i],
//             &lottery_authority,
//         )
//         .await
//         .unwrap();
//     }

//     // Check balances
//     let mut prize_pool_sol_2 = 0.0;
//     for user in &users_wallets {
//         prize_pool_sol_2 += 0.1;
//         check_balance(&mut banks_client, user.pubkey(), 0.8).await;
//     }

//     check_balance(&mut banks_client, holding_wallet.pubkey(), prize_pool_1_remain + prize_pool_sol_2).await;

//     // Finaled lottery
//     let winning_numbers = [1, 1, 1, 1, 1, 1];
//     store_winning_numbers(
//         &mut banks_client,
//         &payer,
//         &recent_blockhash,
//         &winning_numbers,
//         &lottery_authority,
//     )
//     .await
//     .unwrap();

//     let participants = (0..number_of_users)
//         .map(|i| (tickets_pubkeys[i], users_wallets_pubkeys[i]))
//         .collect();

//     // Reward winners
//     let lottery_result = Keypair::new();
//     reward_winners(
//         &mut banks_client,
//         &payer,
//         &recent_blockhash,
//         lottery_result_data_rent,
//         &new_rewards_wallet.pubkey(),
//         &new_slot_holders_rewards_wallet.pubkey(),
//         &new_sollotto_labs_wallet.pubkey(),
//         &charities_pubkeys,
//         &participants,
//         &holding_wallet,
//         &lottery_result,
//         &lottery_authority,
//     )
//     .await
//     .unwrap();

//     // Check balances
//     // lamports_to_sol(1) is division correction
//     check_balance(
//         &mut banks_client,
//         holding_wallet.pubkey(),
//         (prize_pool_sol_2 + prize_pool_1_remain) * 0.65 + lamports_to_sol(1),
//     )
//     .await;
//     check_balance(
//         &mut banks_client,
//         new_rewards_wallet.pubkey(),
//         (prize_pool_sol_2 + prize_pool_1_remain) * 0.04,
//     )
//     .await;
//     check_balance(
//         &mut banks_client,
//         new_slot_holders_rewards_wallet.pubkey(),
//         (prize_pool_sol_2 + prize_pool_1_remain) * 0.006,
//     )
//     .await;
//     check_balance(
//         &mut banks_client,
//         new_sollotto_labs_wallet.pubkey(),
//         (prize_pool_sol_2 + prize_pool_1_remain) * 0.004,
//     )
//     .await;
// }

// #[tokio::test]
// async fn test_without_winners() {
//     let program = ProgramTest::new("sollotto", id(), processor!(Processor::process));
//     let (mut banks_client, payer, recent_blockhash) = program.start().await;

//     let rent = banks_client.get_rent().await.unwrap();
//     let lottery_result_data_rent = rent.minimum_balance(LotteryResultData::LEN);
//     let ticket_data_rent = rent.minimum_balance(TicketData::LEN);
//     let lottery_data_rent = rent.minimum_balance(LotteryData::LEN);

//     let number_of_users = 5;
//     let lottery_id = 112233;
//     let lottery_authority = Keypair::new();
//     let lottery_result = Keypair::new();
//     let holding_wallet = Keypair::new();
//     let rewards_wallet = Keypair::new();
//     let slot_holders_rewards_wallet = Keypair::new();
//     let sollotto_labs_wallet = Keypair::new();
//     let charities: Vec<Keypair> = (0..4).map(|_| Keypair::new()).collect();
//     let charities_pubkeys: Vec<Pubkey> = charities.iter().map(|c| c.pubkey()).collect();
//     let users_wallets: Vec<Keypair> = (0..number_of_users).map(|_| Keypair::new()).collect();
//     let users_wallets_pubkeys: Vec<Pubkey> = users_wallets.iter().map(|x| x.pubkey()).collect();
//     let tickets: Vec<Keypair> = (0..number_of_users).map(|_| Keypair::new()).collect();
//     let tickets_pubkeys: Vec<Pubkey> = tickets.iter().map(|x| x.pubkey()).collect();
//     let winning_numbers = [1, 2, 3, 4, 5, 6];

//     // Set initial balances for users
//     for user in &users_wallets {
//         transfer_sol(&mut banks_client, &recent_blockhash, &payer, user, 1.0)
//             .await
//             .unwrap();
//     }

//     initialize_lottery(
//         &mut banks_client,
//         &payer,
//         &recent_blockhash,
//         lottery_data_rent,
//         lottery_id,
//         &charities_pubkeys,
//         &holding_wallet.pubkey(),
//         &rewards_wallet.pubkey(),
//         &slot_holders_rewards_wallet.pubkey(),
//         &sollotto_labs_wallet.pubkey(),
//         &lottery_authority,
//     )
//     .await
//     .unwrap();

//     // Users purchase tickets
//     let ticket_numbers = [1, 1, 1, 1, 1, 1];
//     for i in 0..number_of_users {
//         purchase_ticket(
//             &mut banks_client,
//             &payer,
//             &recent_blockhash,
//             ticket_data_rent,
//             &charities_pubkeys[0],
//             &ticket_numbers,
//             &holding_wallet.pubkey(),
//             &tickets[i],
//             &users_wallets[i],
//             &lottery_authority,
//         )
//         .await
//         .unwrap();
//     }

//     // Check balances
//     let mut prize_pool_sol = 0.0;
//     for user in &users_wallets {
//         prize_pool_sol += 0.1;
//         check_balance(&mut banks_client, user.pubkey(), 0.9).await;
//     }

//     check_balance(&mut banks_client, holding_wallet.pubkey(), prize_pool_sol).await;

//     // Finaled lottery
//     store_winning_numbers(
//         &mut banks_client,
//         &payer,
//         &recent_blockhash,
//         &winning_numbers,
//         &lottery_authority,
//     )
//     .await
//     .unwrap();

//     let participants = (0..number_of_users)
//         .map(|i| (tickets_pubkeys[i], users_wallets_pubkeys[i]))
//         .collect();

//     // Reward winners
//     reward_winners(
//         &mut banks_client,
//         &payer,
//         &recent_blockhash,
//         lottery_result_data_rent,
//         &rewards_wallet.pubkey(),
//         &slot_holders_rewards_wallet.pubkey(),
//         &sollotto_labs_wallet.pubkey(),
//         &charities_pubkeys,
//         &participants,
//         &holding_wallet,
//         &lottery_result,
//         &lottery_authority,
//     )
//     .await
//     .unwrap();

//     // Check balances
//     check_balance(
//         &mut banks_client,
//         holding_wallet.pubkey(),
//         prize_pool_sol * 0.65,
//     )
//     .await;
//     check_balance(
//         &mut banks_client,
//         rewards_wallet.pubkey(),
//         prize_pool_sol * 0.04,
//     )
//     .await;
//     check_balance(
//         &mut banks_client,
//         slot_holders_rewards_wallet.pubkey(),
//         prize_pool_sol * 0.006,
//     )
//     .await;
//     check_balance(
//         &mut banks_client,
//         sollotto_labs_wallet.pubkey(),
//         prize_pool_sol * 0.004,
//     )
//     .await;

//     // Charity balances
//     check_balance(
//         &mut banks_client,
//         charities_pubkeys[0],
//         prize_pool_sol * 0.3,
//     )
//     .await;
//     for i in 1..4 {
//         check_balance(&mut banks_client, charities_pubkeys[i], 0.0).await;
//     }

//     // Loosers balances
//     for i in 0..5 {
//         check_balance(&mut banks_client, users_wallets_pubkeys[i], 0.9).await;
//     }
// }

// #[tokio::test]
// async fn test_repeat_lottery() {
//     let program = ProgramTest::new("sollotto", id(), processor!(Processor::process));
//     let (mut banks_client, payer, recent_blockhash) = program.start().await;

//     let rent = banks_client.get_rent().await.unwrap();
//     let lottery_result_data_rent = rent.minimum_balance(LotteryResultData::LEN);
//     let ticket_data_rent = rent.minimum_balance(TicketData::LEN);
//     let lottery_data_rent = rent.minimum_balance(LotteryData::LEN);

//     let number_of_users = 5;
//     let lottery_id = 111111;
//     let lottery_authority = Keypair::new();
//     let lottery_result = Keypair::new();
//     let holding_wallet = Keypair::new();
//     let rewards_wallet = Keypair::new();
//     let slot_holders_rewards_wallet = Keypair::new();
//     let sollotto_labs_wallet = Keypair::new();
//     let charities: Vec<Keypair> = (0..4).map(|_| Keypair::new()).collect();
//     let charities_pubkeys: Vec<Pubkey> = charities.iter().map(|c| c.pubkey()).collect();
//     let users_wallets: Vec<Keypair> = (0..number_of_users).map(|_| Keypair::new()).collect();
//     let users_wallets_pubkeys: Vec<Pubkey> = users_wallets.iter().map(|x| x.pubkey()).collect();
//     let tickets: Vec<Keypair> = (0..number_of_users).map(|_| Keypair::new()).collect();
//     let tickets_pubkeys: Vec<Pubkey> = tickets.iter().map(|x| x.pubkey()).collect();
//     let winning_numbers = [1, 2, 3, 4, 5, 6];

//     // Set initial balances for users
//     for user in &users_wallets {
//         transfer_sol(&mut banks_client, &recent_blockhash, &payer, user, 1.0)
//             .await
//             .unwrap();
//     }

//     initialize_lottery(
//         &mut banks_client,
//         &payer,
//         &recent_blockhash,
//         lottery_data_rent,
//         lottery_id,
//         &charities_pubkeys,
//         &holding_wallet.pubkey(),
//         &rewards_wallet.pubkey(),
//         &slot_holders_rewards_wallet.pubkey(),
//         &sollotto_labs_wallet.pubkey(),
//         &lottery_authority,
//     )
//     .await
//     .unwrap();

//     // Users purchase tickets (without winners)
//     let ticket_numbers = [1, 1, 1, 1, 1, 1];
//     for i in 0..number_of_users {
//         purchase_ticket(
//             &mut banks_client,
//             &payer,
//             &recent_blockhash,
//             ticket_data_rent,
//             &charities_pubkeys[0],
//             &ticket_numbers,
//             &holding_wallet.pubkey(),
//             &tickets[i],
//             &users_wallets[i],
//             &lottery_authority,
//         )
//         .await
//         .unwrap();
//     }

//     // Check balances
//     let mut prize_pool_sol = 0.0;
//     for user in &users_wallets {
//         prize_pool_sol += 0.1;
//         check_balance(&mut banks_client, user.pubkey(), 0.9).await;
//     }

//     check_balance(&mut banks_client, holding_wallet.pubkey(), prize_pool_sol).await;

//     // Finaled lottery
//     store_winning_numbers(
//         &mut banks_client,
//         &payer,
//         &recent_blockhash,
//         &winning_numbers,
//         &lottery_authority,
//     )
//     .await
//     .unwrap();

//     let participants = (0..number_of_users)
//         .map(|i| (tickets_pubkeys[i], users_wallets_pubkeys[i]))
//         .collect();

//     // Reward winners
//     reward_winners(
//         &mut banks_client,
//         &payer,
//         &recent_blockhash,
//         lottery_result_data_rent,
//         &rewards_wallet.pubkey(),
//         &slot_holders_rewards_wallet.pubkey(),
//         &sollotto_labs_wallet.pubkey(),
//         &charities_pubkeys,
//         &participants,
//         &holding_wallet,
//         &lottery_result,
//         &lottery_authority,
//     )
//     .await
//     .unwrap();

//     // Check balances
//     let prize_pool_remain = prize_pool_sol * 0.65;
//     check_balance(
//         &mut banks_client,
//         holding_wallet.pubkey(),
//         prize_pool_remain,
//     )
//     .await;
//     let rewards_balance = prize_pool_sol * 0.04;
//     check_balance(&mut banks_client, rewards_wallet.pubkey(), rewards_balance).await;
//     let slot_holders_rewards_balance = prize_pool_sol * 0.006;
//     check_balance(
//         &mut banks_client,
//         slot_holders_rewards_wallet.pubkey(),
//         slot_holders_rewards_balance,
//     )
//     .await;
//     let sollotto_labs_balance = prize_pool_sol * 0.004;
//     check_balance(
//         &mut banks_client,
//         sollotto_labs_wallet.pubkey(),
//         sollotto_labs_balance,
//     )
//     .await;

//     // Charity balances
//     let charity_0_balance = prize_pool_sol * 0.3;
//     check_balance(&mut banks_client, charities_pubkeys[0], charity_0_balance).await;
//     for i in 1..4 {
//         check_balance(&mut banks_client, charities_pubkeys[i], 0.0).await;
//     }

//     // Loosers balances
//     for i in 0..5 {
//         check_balance(&mut banks_client, users_wallets_pubkeys[i], 0.9).await;
//     }

//     // Repeate lottery
//     let lottery_id = 222222;
//     let tickets_2: Vec<Keypair> = (0..number_of_users).map(|_| Keypair::new()).collect();
//     let tickets_pubkeys_2: Vec<Pubkey> = tickets_2.iter().map(|x| x.pubkey()).collect();
//     let winning_numbers_2 = [1, 1, 2, 2, 3, 3];

//     initialize_lottery_without_creation(
//         &mut banks_client,
//         &payer,
//         &recent_blockhash,
//         lottery_id,
//         &charities_pubkeys,
//         &holding_wallet.pubkey(),
//         &rewards_wallet.pubkey(),
//         &slot_holders_rewards_wallet.pubkey(),
//         &sollotto_labs_wallet.pubkey(),
//         &lottery_authority,
//     )
//     .await
//     .unwrap();

//     // Users purchase tickets (user_1 wins)
//     let mut ticket_numbers;
//     for i in 0..number_of_users {
//         if i == 1 {
//             ticket_numbers = winning_numbers_2;
//         } else {
//             ticket_numbers = winning_numbers;
//         }
//         purchase_ticket(
//             &mut banks_client,
//             &payer,
//             &recent_blockhash,
//             ticket_data_rent,
//             &charities_pubkeys[1],
//             &ticket_numbers,
//             &holding_wallet.pubkey(),
//             &tickets_2[i],
//             &users_wallets[i],
//             &lottery_authority,
//         )
//         .await
//         .unwrap();
//     }

//     // Check balances
//     let mut prize_pool_sol_2 = 0.0;
//     for i in 0..number_of_users {
//         prize_pool_sol_2 += 0.1;
//         check_balance(&mut banks_client, users_wallets[i].pubkey(), 0.8).await;
//     }

//     check_balance(
//         &mut banks_client,
//         holding_wallet.pubkey(),
//         prize_pool_remain + prize_pool_sol_2,
//     )
//     .await;

//     // Finaled lottery
//     store_winning_numbers(
//         &mut banks_client,
//         &payer,
//         &recent_blockhash,
//         &winning_numbers_2,
//         &lottery_authority,
//     )
//     .await
//     .unwrap();

//     let participants = (0..number_of_users)
//         .map(|i| (tickets_pubkeys_2[i], users_wallets_pubkeys[i]))
//         .collect();

//     // Reward winners
//     let lottery_result_2 = Keypair::new();
//     reward_winners(
//         &mut banks_client,
//         &payer,
//         &recent_blockhash,
//         lottery_result_data_rent,
//         &rewards_wallet.pubkey(),
//         &slot_holders_rewards_wallet.pubkey(),
//         &sollotto_labs_wallet.pubkey(),
//         &charities_pubkeys,
//         &participants,
//         &holding_wallet,
//         &lottery_result_2,
//         &lottery_authority,
//     )
//     .await
//     .unwrap();

//     // Check balances
//     // lamports_to_sol(1) is division correction
//     let prize_pool_sol_full = prize_pool_remain + prize_pool_sol_2;
//     check_balance(
//         &mut banks_client,
//         holding_wallet.pubkey(),
//         0.0 + lamports_to_sol(1),
//     )
//     .await;
//     check_balance(
//         &mut banks_client,
//         rewards_wallet.pubkey(),
//         rewards_balance + prize_pool_sol_full * 0.04,
//     )
//     .await;
//     check_balance(
//         &mut banks_client,
//         slot_holders_rewards_wallet.pubkey(),
//         slot_holders_rewards_balance + prize_pool_sol_full * 0.006 + lamports_to_sol(1),
//     )
//     .await;
//     check_balance(
//         &mut banks_client,
//         sollotto_labs_wallet.pubkey(),
//         sollotto_labs_balance + prize_pool_sol_full * 0.004,
//     )
//     .await;

//     // Charity balances
//     check_balance(&mut banks_client, charities_pubkeys[0], charity_0_balance).await;
//     check_balance(
//         &mut banks_client,
//         charities_pubkeys[1],
//         prize_pool_sol_full * 0.3,
//     )
//     .await;
//     for i in 2..4 {
//         check_balance(&mut banks_client, charities_pubkeys[i], 0.0).await;
//     }

//     // Loosers balances
//     for i in 0..5 {
//         // Winner balance
//         if i == 1 {
//             check_balance(
//                 &mut banks_client,
//                 users_wallets_pubkeys[1],
//                 prize_pool_sol_full * 0.65 + 0.8,
//             )
//             .await;
//             continue;
//         }

//         check_balance(&mut banks_client, users_wallets_pubkeys[i], 0.8).await;
//     }
// }

// #[tokio::test]
// async fn test_charities_share() {
//     let program = ProgramTest::new("sollotto", id(), processor!(Processor::process));
//     let (mut banks_client, payer, recent_blockhash) = program.start().await;

//     let rent = banks_client.get_rent().await.unwrap();
//     let lottery_result_data_rent = rent.minimum_balance(LotteryResultData::LEN);
//     let ticket_data_rent = rent.minimum_balance(TicketData::LEN);
//     let lottery_data_rent = rent.minimum_balance(LotteryData::LEN);

//     let number_of_users = 5;
//     let lottery_id = 112233;
//     let lottery_authority = Keypair::new();
//     let lottery_result = Keypair::new();
//     let holding_wallet = Keypair::new();
//     let rewards_wallet = Keypair::new();
//     let slot_holders_rewards_wallet = Keypair::new();
//     let sollotto_labs_wallet = Keypair::new();
//     let charities: Vec<Keypair> = (0..4).map(|_| Keypair::new()).collect();
//     let charities_pubkeys: Vec<Pubkey> = charities.iter().map(|c| c.pubkey()).collect();
//     let users_wallets: Vec<Keypair> = (0..number_of_users).map(|_| Keypair::new()).collect();
//     let users_wallets_pubkeys: Vec<Pubkey> = users_wallets.iter().map(|x| x.pubkey()).collect();
//     let tickets: Vec<Keypair> = (0..number_of_users).map(|_| Keypair::new()).collect();
//     let tickets_pubkeys: Vec<Pubkey> = tickets.iter().map(|x| x.pubkey()).collect();
//     let winning_numbers = [1, 2, 3, 4, 5, 6];

//     // Set initial balances for users
//     for user in &users_wallets {
//         transfer_sol(&mut banks_client, &recent_blockhash, &payer, user, 1.0)
//             .await
//             .unwrap();
//     }

//     initialize_lottery(
//         &mut banks_client,
//         &payer,
//         &recent_blockhash,
//         lottery_data_rent,
//         lottery_id,
//         &charities_pubkeys,
//         &holding_wallet.pubkey(),
//         &rewards_wallet.pubkey(),
//         &slot_holders_rewards_wallet.pubkey(),
//         &sollotto_labs_wallet.pubkey(),
//         &lottery_authority,
//     )
//     .await
//     .unwrap();

//     // Users purchase tickets
//     let mut ticket_numbers;
//     let mut user_charity;
//     for i in 0..number_of_users {
//         if i == 0 {
//             ticket_numbers = winning_numbers;
//         } else {
//             ticket_numbers = [1, 1, 1, 1, 1, 1];
//         }

//         if i == 0 || i == 1 {
//             user_charity = charities_pubkeys[0];
//         } else if i == 2 || i == 3 {
//             user_charity = charities_pubkeys[1];
//         } else {
//             user_charity = charities_pubkeys[2];
//         }

//         purchase_ticket(
//             &mut banks_client,
//             &payer,
//             &recent_blockhash,
//             ticket_data_rent,
//             &user_charity,
//             &ticket_numbers,
//             &holding_wallet.pubkey(),
//             &tickets[i],
//             &users_wallets[i],
//             &lottery_authority,
//         )
//         .await
//         .unwrap();
//     }

//     // Check balances
//     let mut prize_pool_sol = 0.0;
//     for user in &users_wallets {
//         prize_pool_sol += 0.1;
//         check_balance(&mut banks_client, user.pubkey(), 0.9).await;
//     }

//     check_balance(&mut banks_client, holding_wallet.pubkey(), prize_pool_sol).await;

//     // Finaled lottery
//     store_winning_numbers(
//         &mut banks_client,
//         &payer,
//         &recent_blockhash,
//         &winning_numbers,
//         &lottery_authority,
//     )
//     .await
//     .unwrap();

//     let participants = (0..number_of_users)
//         .map(|i| (tickets_pubkeys[i], users_wallets_pubkeys[i]))
//         .collect();

//     // Reward winners
//     reward_winners(
//         &mut banks_client,
//         &payer,
//         &recent_blockhash,
//         lottery_result_data_rent,
//         &rewards_wallet.pubkey(),
//         &slot_holders_rewards_wallet.pubkey(),
//         &sollotto_labs_wallet.pubkey(),
//         &charities_pubkeys,
//         &participants,
//         &holding_wallet,
//         &lottery_result,
//         &lottery_authority,
//     )
//     .await
//     .unwrap();

//     // Check balances
//     check_balance(&mut banks_client, holding_wallet.pubkey(), 0.0).await;
//     check_balance(
//         &mut banks_client,
//         rewards_wallet.pubkey(),
//         prize_pool_sol * 0.04,
//     )
//     .await;
//     check_balance(
//         &mut banks_client,
//         slot_holders_rewards_wallet.pubkey(),
//         prize_pool_sol * 0.006,
//     )
//     .await;
//     check_balance(
//         &mut banks_client,
//         sollotto_labs_wallet.pubkey(),
//         prize_pool_sol * 0.004,
//     )
//     .await;

//     // Charity balances (2 charity wins)
//     check_balance(
//         &mut banks_client,
//         charities_pubkeys[0],
//         (prize_pool_sol * 0.3) / 2.0,
//     )
//     .await;
//     check_balance(
//         &mut banks_client,
//         charities_pubkeys[1],
//         (prize_pool_sol * 0.3) / 2.0,
//     )
//     .await;
//     for i in 2..4 {
//         check_balance(&mut banks_client, charities_pubkeys[i], 0.0).await;
//     }

//     // Winner balance
//     check_balance(
//         &mut banks_client,
//         users_wallets_pubkeys[0],
//         prize_pool_sol * 0.65 + 0.9,
//     )
//     .await;
//     // Loosers balances
//     for i in 1..5 {
//         check_balance(&mut banks_client, users_wallets_pubkeys[i], 0.9).await;
//     }
// }

// #[tokio::test]
// async fn test_many_users_many_winners() {
//     let program = ProgramTest::new("sollotto", id(), processor!(Processor::process));
//     let (mut banks_client, payer, recent_blockhash) = program.start().await;

//     let rent = banks_client.get_rent().await.unwrap();
//     let lottery_result_data_rent = rent.minimum_balance(LotteryResultData::LEN);
//     let ticket_data_rent = rent.minimum_balance(TicketData::LEN);
//     let lottery_data_rent = rent.minimum_balance(LotteryData::LEN);

//     let number_of_users = 25;
//     let number_of_winners = 10;
//     let lottery_id = 112233;
//     let lottery_authority = Keypair::new();
//     let lottery_result = Keypair::new();
//     let holding_wallet = Keypair::new();
//     let rewards_wallet = Keypair::new();
//     let slot_holders_rewards_wallet = Keypair::new();
//     let sollotto_labs_wallet = Keypair::new();
//     let charities: Vec<Keypair> = (0..4).map(|_| Keypair::new()).collect();
//     let charities_pubkeys: Vec<Pubkey> = charities.iter().map(|c| c.pubkey()).collect();
//     let users_wallets: Vec<Keypair> = (0..number_of_users).map(|_| Keypair::new()).collect();
//     let users_wallets_pubkeys: Vec<Pubkey> = users_wallets.iter().map(|x| x.pubkey()).collect();
//     let tickets: Vec<Keypair> = (0..number_of_users).map(|_| Keypair::new()).collect();
//     let tickets_pubkeys: Vec<Pubkey> = tickets.iter().map(|x| x.pubkey()).collect();
//     let winning_numbers = [1, 2, 3, 4, 5, 6];
//     let not_winning_numbers = [1, 1, 1, 1, 1, 1];

//     // Set initial balances for users
//     for user in &users_wallets {
//         transfer_sol(&mut banks_client, &recent_blockhash, &payer, user, 1.0)
//             .await
//             .unwrap();
//     }

//     initialize_lottery(
//         &mut banks_client,
//         &payer,
//         &recent_blockhash,
//         lottery_data_rent,
//         lottery_id,
//         &charities_pubkeys,
//         &holding_wallet.pubkey(),
//         &rewards_wallet.pubkey(),
//         &slot_holders_rewards_wallet.pubkey(),
//         &sollotto_labs_wallet.pubkey(),
//         &lottery_authority,
//     )
//     .await
//     .unwrap();

//     // Users purchase tickets
//     let mut ticket_numbers;
//     for i in 0..number_of_users {
//         if i < number_of_winners {
//             ticket_numbers = winning_numbers;
//         } else {
//             ticket_numbers = not_winning_numbers;
//         }

//         purchase_ticket(
//             &mut banks_client,
//             &payer,
//             &recent_blockhash,
//             ticket_data_rent,
//             &charities_pubkeys[0],
//             &ticket_numbers,
//             &holding_wallet.pubkey(),
//             &tickets[i],
//             &users_wallets[i],
//             &lottery_authority,
//         )
//         .await
//         .unwrap();
//     }

//     // Check balances
//     let mut prize_pool_sol = 0.0;
//     for user in &users_wallets {
//         prize_pool_sol += 0.1;
//         check_balance(&mut banks_client, user.pubkey(), 0.9).await;
//     }

//     check_balance(&mut banks_client, holding_wallet.pubkey(), prize_pool_sol).await;

//     // Finaled lottery
//     store_winning_numbers(
//         &mut banks_client,
//         &payer,
//         &recent_blockhash,
//         &winning_numbers,
//         &lottery_authority,
//     )
//     .await
//     .unwrap();

//     let participants = (0..number_of_users)
//         .map(|i| (tickets_pubkeys[i], users_wallets_pubkeys[i]))
//         .collect();

//     // Reward winners
//     reward_winners(
//         &mut banks_client,
//         &payer,
//         &recent_blockhash,
//         lottery_result_data_rent,
//         &rewards_wallet.pubkey(),
//         &slot_holders_rewards_wallet.pubkey(),
//         &sollotto_labs_wallet.pubkey(),
//         &charities_pubkeys,
//         &participants,
//         &holding_wallet,
//         &lottery_result,
//         &lottery_authority,
//     )
//     .await
//     .unwrap();

//     // Check balances
//     check_balance(&mut banks_client, holding_wallet.pubkey(), 0.0).await;
//     check_balance(
//         &mut banks_client,
//         rewards_wallet.pubkey(),
//         prize_pool_sol * 0.04,
//     )
//     .await;
//     check_balance(
//         &mut banks_client,
//         slot_holders_rewards_wallet.pubkey(),
//         prize_pool_sol * 0.006,
//     )
//     .await;
//     check_balance(
//         &mut banks_client,
//         sollotto_labs_wallet.pubkey(),
//         prize_pool_sol * 0.004,
//     )
//     .await;

//     // Charity balances
//     check_balance(
//         &mut banks_client,
//         charities_pubkeys[0],
//         prize_pool_sol * 0.3,
//     )
//     .await;
//     for i in 1..4 {
//         check_balance(&mut banks_client, charities_pubkeys[i], 0.0).await;
//     }

//     // Winner balance
//     for i in 0..number_of_winners {
//         check_balance(
//             &mut banks_client,
//             users_wallets_pubkeys[i],
//             (prize_pool_sol * 0.65) / 10.0 + 0.9,
//         )
//         .await;
//     }
//     // Loosers balances
//     for i in number_of_winners..number_of_users {
//         check_balance(&mut banks_client, users_wallets_pubkeys[i], 0.9).await;
//     }
// }

// #[tokio::test]
// async fn test_many_users_without_winners() {
//     let program = ProgramTest::new("sollotto", id(), processor!(Processor::process));
//     let (mut banks_client, payer, recent_blockhash) = program.start().await;

//     let rent = banks_client.get_rent().await.unwrap();
//     let lottery_result_data_rent = rent.minimum_balance(LotteryResultData::LEN);
//     let ticket_data_rent = rent.minimum_balance(TicketData::LEN);
//     let lottery_data_rent = rent.minimum_balance(LotteryData::LEN);

//     let number_of_users = 30;
//     let lottery_id = 112233;
//     let lottery_authority = Keypair::new();
//     let lottery_result = Keypair::new();
//     let holding_wallet = Keypair::new();
//     let rewards_wallet = Keypair::new();
//     let slot_holders_rewards_wallet = Keypair::new();
//     let sollotto_labs_wallet = Keypair::new();
//     let charities: Vec<Keypair> = (0..4).map(|_| Keypair::new()).collect();
//     let charities_pubkeys: Vec<Pubkey> = charities.iter().map(|c| c.pubkey()).collect();
//     let users_wallets: Vec<Keypair> = (0..number_of_users).map(|_| Keypair::new()).collect();
//     let users_wallets_pubkeys: Vec<Pubkey> = users_wallets.iter().map(|x| x.pubkey()).collect();
//     let tickets: Vec<Keypair> = (0..number_of_users).map(|_| Keypair::new()).collect();
//     let tickets_pubkeys: Vec<Pubkey> = tickets.iter().map(|x| x.pubkey()).collect();
//     let winning_numbers = [1, 2, 3, 4, 5, 6];
//     let not_winning_numbers = [1, 1, 1, 1, 1, 1];

//     // Set initial balances for users
//     for user in &users_wallets {
//         transfer_sol(&mut banks_client, &recent_blockhash, &payer, user, 1.0)
//             .await
//             .unwrap();
//     }

//     initialize_lottery(
//         &mut banks_client,
//         &payer,
//         &recent_blockhash,
//         lottery_data_rent,
//         lottery_id,
//         &charities_pubkeys,
//         &holding_wallet.pubkey(),
//         &rewards_wallet.pubkey(),
//         &slot_holders_rewards_wallet.pubkey(),
//         &sollotto_labs_wallet.pubkey(),
//         &lottery_authority,
//     )
//     .await
//     .unwrap();

//     // Users purchase tickets
//     let mut user_charity;
//     for i in 0..number_of_users {
//         if i < 9 {
//             user_charity = charities_pubkeys[0];
//         } else if i >= 9 && i < 18 {
//             user_charity = charities_pubkeys[2];
//         } else if i >= 18 && i < 27 {
//             user_charity = charities_pubkeys[3];
//         } else {
//             user_charity = charities_pubkeys[1];
//         }

//         purchase_ticket(
//             &mut banks_client,
//             &payer,
//             &recent_blockhash,
//             ticket_data_rent,
//             &user_charity,
//             &not_winning_numbers,
//             &holding_wallet.pubkey(),
//             &tickets[i],
//             &users_wallets[i],
//             &lottery_authority,
//         )
//         .await
//         .unwrap();
//     }

//     // Check balances
//     let mut prize_pool_sol = 0.0;
//     for user in &users_wallets {
//         prize_pool_sol += 0.1;
//         check_balance(&mut banks_client, user.pubkey(), 0.9).await;
//     }

//     check_balance(&mut banks_client, holding_wallet.pubkey(), prize_pool_sol).await;

//     // Finaled lottery
//     store_winning_numbers(
//         &mut banks_client,
//         &payer,
//         &recent_blockhash,
//         &winning_numbers,
//         &lottery_authority,
//     )
//     .await
//     .unwrap();

//     let participants = (0..number_of_users)
//         .map(|i| (tickets_pubkeys[i], users_wallets_pubkeys[i]))
//         .collect();

//     // Reward winners
//     reward_winners(
//         &mut banks_client,
//         &payer,
//         &recent_blockhash,
//         lottery_result_data_rent,
//         &rewards_wallet.pubkey(),
//         &slot_holders_rewards_wallet.pubkey(),
//         &sollotto_labs_wallet.pubkey(),
//         &charities_pubkeys,
//         &participants,
//         &holding_wallet,
//         &lottery_result,
//         &lottery_authority,
//     )
//     .await
//     .unwrap();

//     // Check balances
//     check_balance(
//         &mut banks_client,
//         holding_wallet.pubkey(),
//         prize_pool_sol * 0.65,
//     )
//     .await;
//     check_balance(
//         &mut banks_client,
//         rewards_wallet.pubkey(),
//         prize_pool_sol * 0.04,
//     )
//     .await;
//     check_balance(
//         &mut banks_client,
//         slot_holders_rewards_wallet.pubkey(),
//         prize_pool_sol * 0.006,
//     )
//     .await;
//     check_balance(
//         &mut banks_client,
//         sollotto_labs_wallet.pubkey(),
//         prize_pool_sol * 0.004,
//     )
//     .await;

//     // Charity balances (3 wins)
//     check_balance(
//         &mut banks_client,
//         charities_pubkeys[0],
//         (prize_pool_sol * 0.3) / 3.0,
//     )
//     .await;
//     check_balance(&mut banks_client, charities_pubkeys[1], 0.0).await;
//     check_balance(
//         &mut banks_client,
//         charities_pubkeys[2],
//         (prize_pool_sol * 0.3) / 3.0,
//     )
//     .await;
//     check_balance(
//         &mut banks_client,
//         charities_pubkeys[3],
//         (prize_pool_sol * 0.3) / 3.0,
//     )
//     .await;

//     // Loosers balances
//     for i in 0..number_of_users {
//         check_balance(&mut banks_client, users_wallets_pubkeys[i], 0.9).await;
//     }
// }

// #[tokio::test]
// async fn test_insufficient_funds() {
//     let program = ProgramTest::new("sollotto", id(), processor!(Processor::process));
//     let (mut banks_client, payer, recent_blockhash) = program.start().await;

//     let rent = banks_client.get_rent().await.unwrap();
//     let lottery_result_data_rent = rent.minimum_balance(LotteryResultData::LEN);
//     let ticket_data_rent = rent.minimum_balance(TicketData::LEN);
//     let lottery_data_rent = rent.minimum_balance(LotteryData::LEN);

//     let number_of_users = 5;
//     let lottery_id = 112233;
//     let lottery_authority = Keypair::new();
//     let lottery_result = Keypair::new();
//     let holding_wallet = Keypair::new();
//     let rewards_wallet = Keypair::new();
//     let slot_holders_rewards_wallet = Keypair::new();
//     let sollotto_labs_wallet = Keypair::new();
//     let charities: Vec<Keypair> = (0..4).map(|_| Keypair::new()).collect();
//     let charities_pubkeys: Vec<Pubkey> = charities.iter().map(|c| c.pubkey()).collect();
//     let users_wallets: Vec<Keypair> = (0..number_of_users).map(|_| Keypair::new()).collect();
//     let users_wallets_pubkeys: Vec<Pubkey> = users_wallets.iter().map(|x| x.pubkey()).collect();
//     let tickets: Vec<Keypair> = (0..number_of_users).map(|_| Keypair::new()).collect();
//     let tickets_pubkeys: Vec<Pubkey> = tickets.iter().map(|x| x.pubkey()).collect();
//     let winning_numbers = [1, 2, 3, 4, 5, 6];
//     let not_winning_numbers = [1, 1, 1, 1, 1, 1];

//     initialize_lottery(
//         &mut banks_client,
//         &payer,
//         &recent_blockhash,
//         lottery_data_rent,
//         lottery_id,
//         &charities_pubkeys,
//         &holding_wallet.pubkey(),
//         &rewards_wallet.pubkey(),
//         &slot_holders_rewards_wallet.pubkey(),
//         &sollotto_labs_wallet.pubkey(),
//         &lottery_authority,
//     )
//     .await
//     .unwrap();

//     // Create tickets accounts
//     for i in 0..number_of_users {
//         create_ticket_account(
//             &mut banks_client,
//             &payer,
//             &recent_blockhash,
//             ticket_data_rent,
//             &tickets[i],
//         )
//         .await
//         .unwrap();
//     }

//     // Users try to purchase tickets
//     for i in 0..number_of_users {
//         assert_eq!(
//             TransactionError::InstructionError(0, InstructionError::InsufficientFunds),
//             purchase_ticket_without_creation(
//                 &mut banks_client,
//                 &payer,
//                 &recent_blockhash,
//                 &charities_pubkeys[0],
//                 &not_winning_numbers,
//                 &holding_wallet.pubkey(),
//                 &tickets[i],
//                 &users_wallets[i],
//                 &lottery_authority,
//             )
//             .await
//             .unwrap_err()
//             .unwrap(),
//         );
//     }

//     // Set initial balances for users
//     for i in 0..number_of_users {
//         transfer_sol(
//             &mut banks_client,
//             &recent_blockhash,
//             &payer,
//             &users_wallets[i],
//             1.0,
//         )
//         .await
//         .unwrap();
//     }

//     // Users purchase tickets
//     for i in 0..number_of_users {
//         purchase_ticket_without_creation(
//             &mut banks_client,
//             &payer,
//             &recent_blockhash,
//             &charities_pubkeys[1],
//             &not_winning_numbers,
//             &holding_wallet.pubkey(),
//             &tickets[i],
//             &users_wallets[i],
//             &lottery_authority,
//         )
//         .await
//         .unwrap();
//     }

//     // Finaled lottery
//     store_winning_numbers(
//         &mut banks_client,
//         &payer,
//         &recent_blockhash,
//         &winning_numbers,
//         &lottery_authority,
//     )
//     .await
//     .unwrap();

//     let participants = (0..number_of_users)
//         .map(|i| (tickets_pubkeys[i], users_wallets_pubkeys[i]))
//         .collect();

//     // Spend prize pool from holding wallet acc
//     transfer_sol(
//         &mut banks_client,
//         &recent_blockhash,
//         &holding_wallet,
//         &payer,
//         0.4,
//     )
//     .await
//     .unwrap();

//     // Try to Reward winners
//     assert_eq!(
//         TransactionError::InstructionError(1, InstructionError::InsufficientFunds),
//         reward_winners(
//             &mut banks_client,
//             &payer,
//             &recent_blockhash,
//             lottery_result_data_rent,
//             &rewards_wallet.pubkey(),
//             &slot_holders_rewards_wallet.pubkey(),
//             &sollotto_labs_wallet.pubkey(),
//             &charities_pubkeys,
//             &participants,
//             &holding_wallet,
//             &lottery_result,
//             &lottery_authority,
//         )
//         .await
//         .unwrap_err()
//         .unwrap(),
//     );
// }
