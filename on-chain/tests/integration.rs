// use solana_program::{program_pack::Pack, system_instruction};
// use solana_program_test::*;
// use solana_sdk::signature::Keypair;
// use sollotto::{processor::Processor, processor::id, state::{LotteryData, TicketData}};
// use {
//     solana_program::{
//         instruction::{AccountMeta, Instruction},
//         pubkey::Pubkey,
//     },
//     solana_sdk::{signature::Signer, transaction::Transaction},
//     solana_validator::test_validator::*,
// };

// #[tokio::test]
// async fn test_validator_transaction() {
//     // TODO: this is all just for test now
//     let program_test = ProgramTest::new("sollotto", id(), processor!(Processor::process));

//     let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

//     let user_1 = Keypair::new();
//     let user_2 = Keypair::new();
//     let user_3 = Keypair::new();
//     let user_4 = Keypair::new();
//     let user_5 = Keypair::new();
//     let user_6 = Keypair::new();
//     let user_7 = Keypair::new();
//     let user_8 = Keypair::new();
//     let user_9 = Keypair::new();
//     let user_10 = Keypair::new();
//     let user_11 = Keypair::new();
//     let user_12 = Keypair::new();
//     let user_13 = Keypair::new();
//     let user_14 = Keypair::new();
//     let user_15 = Keypair::new();
//     let user_16 = Keypair::new();
//     let user_17 = Keypair::new();
//     let user_18 = Keypair::new();
//     let user_19 = Keypair::new();
//     let user_20 = Keypair::new();
//     let user_21 = Keypair::new();
//     let user_22 = Keypair::new();
//     let user_23 = Keypair::new();
//     let user_24 = Keypair::new();
//     let user_25 = Keypair::new();
//     let participants = vec![
//         user_1, user_2, user_3, user_4, user_5,
//         user_6, user_7, user_8, user_9, user_10,
//         user_11, user_12, user_13, user_14, user_15,
//         user_16, user_17, user_18, user_19, user_20,
//         user_21, user_22, user_23, user_24, user_25,
//     ];
//     let lottery_result = Keypair::new();
//     let lottery_auth = Keypair::new();

//     let rent = banks_client.get_rent().await.unwrap();

//     let mut transaction = Transaction::new_with_payer(
//         &[
//             system_instruction::create_account(
//                 &payer.pubkey(),
//                 &lottery_auth.pubkey(),
//                 rent.minimum_balance(LotteryData::LEN),
//                 sollotto::state::LotteryData::LEN as u64,
//                 &id(),
//             ),
//             sollotto::instruction::initialize_lottery(
//             &id(),
//             1,
//             1,
//             2,
//             3,
//             4,
//             &lottery_auth.pubkey(),
//         )
//         .unwrap()],
//         Some(&payer.pubkey()),
//     );
//     transaction.sign(
//         &[&payer, &lottery_auth],
//         recent_blockhash,
//     );
//     banks_client.process_transaction(transaction).await.unwrap();

//     let mut i = 0;
//     for part in &participants {
//         i+=1;
//         let mut transaction = Transaction::new_with_payer(
//             &[
//                 system_instruction::create_account(
//                     &payer.pubkey(),
//                     &part.pubkey(),
//                     rent.minimum_balance(TicketData::LEN),
//                     sollotto::state::TicketData::LEN as u64,
//                     &id(),
//                 ),
//                 sollotto::instruction::purchase_ticket(
//                 &id(),
//                 1,
//                 &part.pubkey(),
//                 &[i,2,3,4,5,6],
//                 &lottery_auth.pubkey(),
//             )
//             .unwrap()],
//             Some(&payer.pubkey()),
//         );
//         transaction.sign(
//             &[&payer, &lottery_auth, &par],
//             recent_blockhash,
//         );
//         banks_client.process_transaction(transaction).await.unwrap();
//     }

//     let participants_pubkeys: Vec<Pubkey> = participants.iter().map(|k| k.pubkey()).collect();
//     let mut transaction = Transaction::new_with_payer(
//         &[sollotto::instruction::reward_winners(
//             &id(),
//             &participants_pubkeys,
//             &lottery_result.pubkey(),
//             &lottery_auth.pubkey(),
//         )
//         .unwrap()],
//         Some(&payer.pubkey()),
//     );
//     transaction.sign(
//         &[&payer, &lottery_auth],
//         recent_blockhash,
//     );
//     banks_client.process_transaction(transaction).await.unwrap();
// }
