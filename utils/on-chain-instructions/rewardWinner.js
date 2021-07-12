const {
  Account,
  SystemProgram,
  TransactionInstruction,
  Transaction,
  sendAndConfirmRawTransaction,
} = require("@solana/web3.js");
const { connection } = require("../../config");

const rewardWinner = async (
  lotteryId,
  lotteryDataAccount,
  charities,
  winningNumberArr
) => {
  //getting Lottery Data Account for signing

  const lotteryBytes = CryptoJS.AES.decrypt(
    lotteryDataAccount,
    process.env.SECRET_KEY
  );
  const lotteryDecryptedText = JSON.parse(
    lotteryBytes.toString(CryptoJS.enc.Utf8)
  );
  const LotteryDataAccount = new Account(lotteryDecryptedText);

  //getting Holding Wallet for signing

  const HoldingWalletBytes = CryptoJS.AES.decrypt(
    process.env.HOLDING_WALLET_SECRETKEY,
    process.env.SECRET_KEY
  );
  const HoldingWalletDecryptedText = JSON.parse(
    HoldingWalletBytes.toString(CryptoJS.enc.Utf8)
  );

  const HoldingWallet = new Account(HoldingWalletDecryptedText);
  console.log(HoldingWallet.publicKey);
  
  //getting other wallets for Instruction

  const rewards_wallet = new PublicKey(process.env.REWARD_WALLET_PUBLIC_KEY);
  const slot_holders_rewards_wallet = new PublicKey(
    process.env.SLOT_HOLDER_REWARDS_PUBLIC_KEY
  );
  const sollotto_labs_wallet = new PublicKey(
    process.env.SOLLOTTO_LABS_PUBLIC_KEY
  );

  //getting program id

  const solanaProgramId = new PublicKey(
    process.env.SOLANA_INIT_LOTTERY_PROGRAM
  );


try {
  const data = {
    lottery_id: lotteryId,
    winning_numbers: winningNumberArr,
  };
  const dataArr = Buffer.from(JSON.stringify(data));

  const lotteryResultDataAccount = new Account();

  const lotteryResultDataAccountTx = SystemProgram.createAccount({
    space: 10,
    lamports: await connection.getMinimumBalanceForRentExemption(
      10,
      "singleGossip"
    ),
    fromPubkey: HoldingWallet.publicKey,
    newAccountPubkey: lotteryResultDataAccount.publicKey,
    programId: solanaProgramId,
  });

  const rewardWinnerTx = new TransactionInstruction({
    programId: solanaProgramId,
    keys: [
      {
        pubkey: LotteryDataAccount.publicKey,
        isSigner: true,
        isWritable: true,
      },
      {
        pubkey: lotteryResultDataAccount.publicKey,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: HoldingWallet.publicKey,
        isSigner: true,
        isWritable: true,
      },
      {
        pubkey: rewards_wallet,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: slot_holders_rewards_wallet,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: sollotto_labs_wallet,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: charities[0].publicKey,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: charities[1].publicKey,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: charities[2].publicKey,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: charities[3].publicKey,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey:solanaProgramId,
        isSigner: false,
        isWritable: false,
      },
      {
        //publickey: dont know what to add in n^2 participants
        isSigner: false,
        isWritable: false,
      },
    ],
    data: dataArr,
  });
  const transaction = new Transaction(
    lotteryResultDataAccountTx,
    rewardWinnerTx
  );

  const confirmation = await connection.sendTransaction(
    connection,
    transaction,
    [HoldingWallet, LotteryDataAccount],
    {
      commitment: "singleGossip",
      preflightCommitment: "singleGossip",
    }
  );
  return confirmation;
} catch (e) {
  console.log(e);
}
};
module.exports = {
  rewardWinner,
};
