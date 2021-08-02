use solana_program::{
    hash::Hash,
    instruction::InstructionError,
    native_token::{sol_to_lamports, lamports_to_sol},
    program_pack::Pack,
    system_instruction::{self},
    msg
};
use solana_program_test::*;
use solana_sdk::{
    signature::Keypair, system_transaction, transaction::TransactionError,
    transport::TransportError,
};
use sollotto_model_5::{
    processor::id,
    processor::Processor,
    state::LotteryResultData,
};
use spl_token::state::{Account, Mint};
use {
    solana_program::pubkey::Pubkey,
    solana_sdk::{signature::Signer, transaction::Transaction},
};

// Helper functions
//
async fn initialize_lottery(
    banks_client: &mut BanksClient,
    payer: &Keypair,
    recent_blockhash: &Hash,
    mint_rent: u64,
    token_account_rent: u64,
    sollotto_key: &Keypair,
    fqticket_mint: &Keypair,
    fqticket_mint_authority: &Keypair,
    slot_mint: &Keypair,
    slot_mint_authority: &Keypair,
    sollotto_sol: &Keypair
) -> Result<(), TransportError> {
    let mut transaction = Transaction::new_with_payer(
        &[
            system_instruction::create_account(
                &payer.pubkey(),
                &fqticket_mint.pubkey(),
                mint_rent,
                spl_token::state::Mint::LEN as u64,
                &spl_token::id(),
            ),
            spl_token::instruction::initialize_mint(
                &spl_token::id(),
                &fqticket_mint.pubkey(),
                &fqticket_mint_authority.pubkey(),
                None,
                9
            ).unwrap(),
            system_instruction::create_account(
                &payer.pubkey(),
                &slot_mint.pubkey(),
                mint_rent,
                spl_token::state::Mint::LEN as u64,
                &spl_token::id(),
            ),
            spl_token::instruction::initialize_mint(
                &spl_token::id(),
                &slot_mint.pubkey(),
                &slot_mint_authority.pubkey(),
                None,
                9
            ).unwrap(),
        ],
        Some(&payer.pubkey()),
    );
    transaction.sign(
        &[
            payer,
            fqticket_mint,
            slot_mint,
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

async fn allocate_account(
    banks_client: &mut BanksClient,
    payer: &Keypair,
    recent_blockhash: &Hash,
    account: &Keypair,
    space: u64
) -> Result<(), TransportError> {
    let mut transaction = Transaction::new_with_payer(
        &[ system_instruction::allocate(&account.pubkey(), space) ],
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

async fn mint_token(
    banks_client: &mut BanksClient,
    recent_blockhash: &Hash,
    payer: &Keypair,
    mint: &Keypair,
    to: &Pubkey,
    owner: &Pubkey,
    amount: u64
) -> Result<(), TransportError> {
    let mut transaction = Transaction::new_with_payer(
        &[spl_token::instruction::mint_to(
            &spl_token::id(),
            &mint.pubkey(),
            &to,
            &owner,
            &[&mint.pubkey()],
            amount)
        .unwrap()],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[payer, mint], *recent_blockhash);
    banks_client.process_transaction(transaction).await?;
    Ok(())
}

async fn reward_winner(
    banks_client: &mut BanksClient,
    recent_blockhash: &Hash,
    payer: &Keypair,
    lottery_id: u32,
    idx: u64,
    prize_pool: u64,
    sollotto_sol: &Keypair,
    sollotto_rewards: &Pubkey,
    slot_holder_rewards: &Pubkey,
    sollotto_labs: &Pubkey,
    sollotto_result: &Keypair,
    participants: &Vec<(Pubkey, Pubkey)>
) -> Result<(), TransportError> {
    let mut transaction = Transaction::new_with_payer(
        &[sollotto_model_5::instruction::reward_winners(
            &id(),
            lottery_id,
            idx,
            prize_pool,
            &sollotto_sol.pubkey(),
            &sollotto_rewards,
            &slot_holder_rewards,
            &sollotto_labs,
            &sollotto_result.pubkey(),
            &participants
        )
        .unwrap()],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[payer, sollotto_sol, sollotto_result], *recent_blockhash);
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
async fn test_reward_winners() -> Result<(), Box<std::error::Error>> {
    let program = ProgramTest::new("sollotto", id(), processor!(Processor::process));
    let (mut banks_client, payer, recent_blockhash) = program.start().await;
    let rent = banks_client.get_rent().await.unwrap();
    let mint_rent = rent.minimum_balance(Mint::LEN);
    let token_account_rent = rent.minimum_balance(Account::LEN);
    let lottery_result_rent = rent.minimum_balance(LotteryResultData::LEN);

    let sollotto_key            = Keypair::new();
    let sollotto_sol            = Keypair::new();
    let fqticket_mint           = Keypair::new();
    let fqticket_mint_authority = Keypair::new();
    let slot_mint               = Keypair::new();
    let slot_mint_authority     = Keypair::new();
    let sollotto_rewards        = Keypair::new();
    let slot_holder_rewards     = Keypair::new();
    let sollotto_labs           = Keypair::new();
    let sollotto_result         = Keypair::new();

    let n_participants = 4;
    let participants_sol: Vec<Keypair> =
        (0..n_participants)
        .map(|_| Keypair::new())
        .collect();

    let participants_slot: Vec<Keypair> =
        (0..n_participants)
        .map(|_| Keypair::new())
        .collect();

    let participants_fqticket: Vec<Keypair> =
        (0..n_participants)
        .map(|_| Keypair::new())
        .collect();

    initialize_lottery(
        &mut banks_client,
        &payer,
        &recent_blockhash,
        mint_rent,
        token_account_rent,
        &sollotto_key,
        &fqticket_mint,
        &fqticket_mint_authority,
        &slot_mint,
        &slot_mint_authority,
        &sollotto_sol
    ).await.unwrap();

    for i in 0..n_participants {
        create_token_account(
            &mut banks_client,
            &payer,
            &recent_blockhash,
            &participants_fqticket[i],
            token_account_rent,
            &fqticket_mint.pubkey(),
            &participants_sol[i].pubkey()
        )
        .await
        .unwrap();

        create_token_account(
            &mut banks_client,
            &payer,
            &recent_blockhash,
            &participants_slot[i],
            token_account_rent,
            &slot_mint.pubkey(),
            &participants_sol[i].pubkey()
        )
        .await
        .unwrap();

        /* FIXME: This throws an error for some reason
        mint_token(
            &mut banks_client,
            &recent_blockhash,
            &payer,
            &slot_mint,
            &participants_slot[i].pubkey(),
            &participants_sol[i].pubkey(),
            10
        )
        .await
        .unwrap();
        */
    }

    // Assert that created wallets have 0 SOL
    for i in 0..n_participants {
        check_balance(&mut banks_client, participants_sol[i].pubkey(), 0.).await;
    }

    let lottery_id = 1;
    let winning_idx = 2;
    let prize_pool = 50.;

    transfer_sol(&mut banks_client, &recent_blockhash, &payer, &sollotto_sol, prize_pool).await?;

    reward_winner(
        &mut banks_client,
        &recent_blockhash,
        &payer,
        lottery_id,
        winning_idx,
        sol_to_lamports(prize_pool),
        &sollotto_sol,
        &sollotto_rewards.pubkey(),
        &slot_holder_rewards.pubkey(),
        &sollotto_labs.pubkey(),
        &sollotto_result,
        &participants_sol.iter()
            .zip(participants_fqticket)
            .map(|(fst, scnd)| { (fst.pubkey(), scnd.pubkey()) })
            .collect()
    ).await?;

    check_balance(
        &mut banks_client,
        participants_sol[winning_idx as usize].pubkey(),
        (sol_to_lamports(prize_pool) as f64) * 0.95
    );

    check_balance(
        &mut banks_client,
        sollotto_rewards.pubkey(),
        (sol_to_lamports(prize_pool) as f64) * 0.04
    );

    check_balance(
        &mut banks_client,
        slot_holder_rewards.pubkey(),
        (sol_to_lamports(prize_pool) as f64) * 0.006
    );

    check_balance(
        &mut banks_client,
        sollotto_labs.pubkey(),
        (sol_to_lamports(prize_pool) as f64) * 0.004
    );

    Ok(())
}
