const {
  Account,
  SystemProgram,
  TransactionInstruction,
  Transaction,
  sendAndConfirmRawTransaction,
} = require("@solana/web3.js");
var CryptoJS = require("crypto-js");
const { connection } = require("../../config");

const storeWinningNumbers = async (lotteryDataAccount, winningNumberArr) => {

  //getting LotteryDataAccount for signing the transaction

  const lotteryBytes = CryptoJS.AES.decrypt(
    lotteryDataAccount,
    process.env.SECRET_KEY
  );
  const lotteryDecryptedText = JSON.parse(
    lotteryBytes.toString(CryptoJS.enc.Utf8)
  );
  const LotteryDataAccount = new Account(lotteryDecryptedText);

  // getting holding wallet account for creating data account

  const HoldingWalletBytes = CryptoJS.AES.decrypt(
    process.env.HOLDING_WALLET_SECRETKEY,
    process.env.SECRET_KEY
  );
  const HoldingWalletDecryptedText = JSON.parse(
    HoldingWalletBytes.toString(CryptoJS.enc.Utf8)
  );
  const HoldingWallet = new Account(HoldingWalletDecryptedText);

  //getting program id

  const solanaProgramId = new PublicKey(
    process.env.SOLANA_INIT_LOTTERY_PROGRAM
  );
  try {
    //creating the buffer to give it to instruction
    const data = {
      winning_numbers_arr: winningNumberArr,
    };
    const dataArr = new BufferLayout.Blob(6,data)

    // creating a new account for instruction
    const storeWinningNumbers = new Account();
    const createStoreWinningNumbersTx = SystemProgram.createAccount({
      space: 6,
      lamports: await connection.getMinimumBalanceForRentExemption(
        6,
        "singleGossip"
      ),
      fromPubkey: HoldingWallet.publicKey,
      newAccountPubkey: storeWinningNumbers.publicKey,
      programId: solanaProgramId,
    });

    const storeWinningNumbersTx = new TransactionInstruction({
      programId: solanaProgramId,
      keys: [
        {
          pubkey: LotteryDataAccount.publicKey,
          isSigner: true,
          isWritable: true,
        },
      ],
      data:dataArr
    });

    var transaction = new Transaction().add(
      createStoreWinningNumbersTx,
      storeWinningNumbersTx
    );
    await sendAndConfirmRawTransaction(
      connection,
      transaction,
      [HoldingWallet, LotteryDataAccount],
      {
        commitment: "singleGossip",
        preflightCommitment: "singleGossip",
      }
    );
  } catch (e) {
    console.log(e);
  }
};

module.exports = {
  storeWinningNumbers,
};
