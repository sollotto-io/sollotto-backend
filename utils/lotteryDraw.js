const {
  PublicKey,
  SYSVAR_RENT_PUBKEY,
  LAMPORTS_PER_SOL,
  Connection,
} = require("@solana/web3.js");
var borsh = require("borsh");
const sleep = require("util").promisify(setTimeout);
const { TicketDataAccount, TicketDataSchema } = require("./TicketDataBorsh.js");
const {
  LotteryDataAccount,
  LotteryDataSchema,
} = require("./LotteryDataBorsh.js");
var random = require("random");
const ticket = require("../models/ticket.js");
const dotenv = require("dotenv");
dotenv.config();
const lotteryDraw = async (data) => {
  const tickets = await ticket.find({ LotteryId: data.Id });
  var lotteryDataAcc = [];
  tickets.map((t) => {
    lotteryDataAcc.push(t.DataWallet);
  });
  let connection = new Connection("https://api.devnet.solana.com");
  let lotteryDataAccountPK = data.LotteryDataAccount;
  let ticketDataAccountPKArr = lotteryDataAcc;
  let winnerUserTicketDataWalletsPK = [];
  let winnerUserWalletsPK = [];
  let winningNumberArr = [
    random.int(1, 69),
    random.int(1, 69),
    random.int(1, 69),
    random.int(1, 69),
    random.int(1, 69),
    random.int(1, 26),
  ];
  let winFlag = false;
  var usersTicketNumberArr = [];
  var interval = 250;
  var promise = Promise.resolve();
  await ticketDataAccountPKArr.forEach(async function (publicKey,i) {
    promise = promise.then(async function () {

      const encodedTicketDataState = await connection.getAccountInfo(
        new PublicKey(publicKey),
        "singleGossip"
      );
      const decodedTicketDataState = borsh.deserialize(
        TicketDataSchema,
        TicketDataAccount,
        encodedTicketDataState.data
      );
      usersTicketNumberArr.push(
        decodedTicketDataState.charity_id.ticket_number_arr
      );
      return new Promise(function (resolve) {
        setTimeout(resolve, interval);
      });
    });
  });

  promise.then(async function () {
	await usersTicketNumberArr.forEach((numberArr, index) => {
		if (numberArr === winningNumberArr) {
		  winnerUserTicketDataWalletsPK.push(ticketDataAccountPKArr[index]);
		  winFlag = true;
		}
	  });
  });

  // let usersTicketNumberArr = ticketDataAccountPKArr.map( async (publicKey) => {
  // 	const encodedTicketDataState = await connection.getAccountInfo(
  // 		new PublicKey(publicKey),
  // 		"singleGossip"
  // 	);
  // 	const decodedTicketDataState = borsh.deserialize(
  // 		TicketDataSchema,
  // 		TicketDataAccount,
  // 		encodedTicketDataState.data
  // 	);
  // 	return decodedTicketDataState.charity_id.ticket_number_arr;

  // });

 
   

  let encodedLotteryDataState = await connection.getAccountInfo(
    new PublicKey(lotteryDataAccountPK),
    "singleGossip"
  );
  let decodedLotteryDataState = borsh.deserialize(
    LotteryDataSchema,
    LotteryDataAccount,
    encodedLotteryDataState.data
  );
  let charityVC = [];
  charityVC.push(decodedLotteryDataState.is_lottery_initialised.charity_1_vc);
  charityVC.push(decodedLotteryDataState.is_lottery_initialised.charity_2_vc);
  charityVC.push(decodedLotteryDataState.is_lottery_initialised.charity_3_vc);
  charityVC.push(decodedLotteryDataState.is_lottery_initialised.charity_4_vc);

  let winningLotteryIndexes = [0];
  await charityVC.forEach((value, index) => {
    if (index > 0) {
      if (value > charityVC[winningLotteryIndexes[0]]) {
        winningLotteryIndexes = [index];
      } else if (value === charityVC[winningLotteryIndexes[0]]) {
        winningLotteryIndexes.push(index);
      }
    }
  });
  let winningCharities = [];
  await winningLotteryIndexes.forEach((winIndex) => {
    if (winIndex === 0) {
      winningCharities.push(
        decodedLotteryDataState.is_lottery_initialised.charity_1_id
      );
    }
    if (winIndex === 1) {
      winningCharities.push(
        decodedLotteryDataState.is_lottery_initialised.charity_2_id
      );
    }
    if (winIndex === 2) {
      winningCharities.push(
        decodedLotteryDataState.is_lottery_initialised.charity_3_id
      );
    }
    if (winIndex === 3) {
      winningCharities.push(
        decodedLotteryDataState.is_lottery_initialised.charity_4_id
      );
    }
  });

  if (winFlag === true) {
    await winnerUserTicketDataWalletsPK.forEach(async (publicKey) => {
      let encodedWinnerTicketDataState = await connection.getAccountInfo(
        new PublicKey(publicKey),
        "singleGossip"
      );
      let decodedWinnerTicketDataState = borsh.deserialize(
        TicketDataSchema,
        TicketDataAccount,
        encodedWinnerTicketDataState.data
      );
      winnerUserWalletsPK.push(
        decodedWinnerTicketDataState.charity_id.user_wallet_pk
      );
    });
    return { winFlag, winningNumberArr, winnerUserWalletsPK, winningCharities };
  } else if (winFlag === false) {
    return { winFlag, winningNumberArr, winningCharities };
  }
};

module.exports = {
  lotteryDraw,
};
