const {
  PublicKey,
  SYSVAR_RENT_PUBKEY,
  LAMPORTS_PER_SOL,
  Connection,
} = require("@solana/web3.js");
var borsh = require("borsh");
const sleep = require("util").promisify(setTimeout);
const { TicketDataAccount, TicketDataSchema } = require("./TicketDataBorsh.js");
const _  = require('lodash')
var random = require("random");
const dotenv = require("dotenv");
dotenv.config();
const lotteryDraw = async (data) => {
  var lotteryDataAcc = [];
  await data.Tickets.map((t) => {
    lotteryDataAcc.push(t.DataWallet);
  });

  let connection = new Connection("https://api.devnet.solana.com");
  let ticketDataAccountPKArr = lotteryDataAcc;
  let winnerUserTicketDataWalletsPK = [];
  let winnerUserWalletsPK = [];
  let winningNumberArr = [
    1,2,3,4,5,6
    // random.int(1, 69),
    // random.int(1, 69),
    // random.int(1, 69),
    // random.int(1, 69),
    // random.int(1, 69),
    // random.int(1, 26),
  ];
  let winFlag = false;
 
  await ticketDataAccountPKArr.forEach(async function (publicKey,i) {
      setTimeout(async()=>{
        const encodedTicketDataState = await connection.getAccountInfo(
          		new PublicKey(publicKey),
          		"singleGossip"
          	);
          	const decodedTicketDataState = await borsh.deserialize(
          		TicketDataSchema,
          		TicketDataAccount,
          		encodedTicketDataState.data
          	);
            if (_.isEqual(Buffer.from(decodedTicketDataState.charity_id.ticket_number_arr).toJSON().data, winningNumberArr)) {
              await winnerUserTicketDataWalletsPK.push(ticketDataAccountPKArr[i]);
              winFlag = true;
            }
            
            console.log(winnerUserTicketDataWalletsPK, winFlag)
      }, i *1000 )
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
  
    return { winFlag, winningNumberArr, winnerUserWalletsPK };
  } else if (winFlag === false) {
    return { winFlag, winningNumberArr };
  }
};

module.exports = {
  lotteryDraw,
};
