const {
  SystemProgram,
  Transaction,
  TransactionInstruction,
  Account,
  SYSVAR_RENT_PUBKEY,
  PublicKey,
  sendAndConfirmTransaction,
} = require("@solana/web3.js");
var CryptoJS = require("crypto-js");
const { connection } = require("../../config.js");
const BufferLayout  =  require('buffer-layout')

const initLottery = async (charities) => {
  
 
  // getting the signerAccount details

  const HoldingWalletBytes = CryptoJS.AES.decrypt(process.env.HOLDING_WALLET_SECRETKEY, process.env.SECRET_KEY);
  const HoldingWalletDecryptedText = JSON.parse(HoldingWalletBytes.toString(CryptoJS.enc.Utf8));
  const HoldingWallet = new Account(HoldingWalletDecryptedText);


  //getting public keys for data....

  const rewards_wallet = new PublicKey(process.env.REWARD_WALLET_PUBLIC_KEY)
  const slot_holders_rewards_wallet = new PublicKey(process.env.SLOT_HOLDER_REWARDS_PUBLIC_KEY)
  const sollotto_labs_wallet = new PublicKey(process.env.SOLLOTTO_LABS_PUBLIC_KEY )

  //Solana Program id public key

  const solanaProgramId = new PublicKey(
    process.env.SOLANA_INIT_LOTTERY_PROGRAM
  );


  try {
    console.log("working")
 // creating the object for Instruction data

    const lotteryFields = {
      lottery_id: parseInt(Math.floor(new Date().valueOf() * Math.random())/100000),
      charity_1: new PublicKey(charities[0].publickey).toBytes(),
      charity_2: new PublicKey(charities[1].publickey).toBytes(),
      charity_3: new PublicKey(charities[2].publickey).toBytes(),
      charity_4: new PublicKey(charities[3].publickey).toBytes(),
      holding_wallet:HoldingWallet.publicKey,
      rewards_wallet:rewards_wallet,
      slot_holders_rewards_wallet:slot_holders_rewards_wallet,
      sollotto_labs_wallet:sollotto_labs_wallet
    
    }
    //converting data into Buffer to be passed in instruction

    dataArr = new BufferLayout.Blob(296,lotteryFields)

    //create a new lotteryData account

    const lotteryDataAccount = new Account();

    //creating data account for lottery

    const createLotteryDataAccountTx = SystemProgram.createAccount({
      space: 296,
      lamports: await connection.getMinimumBalanceForRentExemption(
        296,
        "singleGossip"
      ),
      fromPubkey: HoldingWallet.publicKey,
      newAccountPubkey: lotteryDataAccount.publicKey,
      programId: solanaProgramId,
    });
   
    //creating the transaction instruction fro lottery data account

    const initLotteryTx = new TransactionInstruction({
      programId: solanaProgramId,
      keys: [
        {
          pubkey: lotteryDataAccount.publicKey,
          isSigner: true,
          isWritable: true,
        },
        { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
      ],
      data: dataArr,
    });

    // creating transaction 

    const transaction = new Transaction().add(
      createLotteryDataAccountTx,
      initLotteryTx
    );
    //sending and confirming the transaction 
    
    await sendAndConfirmTransaction(
      connection,
      transaction,
      [HoldingWallet, lotteryDataAccount],
      {
        commitment: "singleGossip",
        preflightCommitment: "singleGossip",
      }
    );
  var lotteryDataAccountSKString = CryptoJS.AES.encrypt(JSON.stringify(Buffer.from(lotteryDataAccount.secretKey).toJSON().data), process.env.SECRET_KEY).toString();
    return {lotteryDataSK : lotteryDataAccountSKString, lotteryId:lotteryFields.lottery_id };
  } catch (e) {
    console.warn(e);
    console.log(`Error: ${e.message}`);
  }
};
initLottery();
module.exports = {
  initLottery,
};
