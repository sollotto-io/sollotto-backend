const {
	SystemProgram,
	PublicKey,
	Transaction,
	TransactionInstruction,
	Account,
	SYSVAR_RENT_PUBKEY,
	LAMPORTS_PER_SOL,
	Connection,
} = require("@solana/web3.js");
var borsh = require("borsh");
const { TicketDataAccount, TicketDataSchema } = require("./TicketDataBorsh.js");
const {
	LotteryDataAccount,
	LotteryDataSchema,
} = require("./LotteryDataBorsh.js");
var random = require("random");

const lotteryDraw = async (data) => {
	let connection = new Connection("https://devnet.solana.com");
	
	let totalPool = null; // fetch total pool of draw lottery
	let lotteryDataAccountPK = []; //Fetch ticketDataAccountPK of draw lottery
	let ticketDataAccountPKArr = []; // fetch all user ticketDataAccountPK of draw lottery
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

	// Fetch DataWallet
	let usersTicketNumberArr = ticketDataAccountPKArr.map(async (publicKey) => {
		const encodedTicketDataState = await connection.getAccountInfo(
			new PublicKey(publicKey),
			"singleGossip"
		);
		const decodedTicketDataState = borsh.deserialize(
			TicketDataSchema,
			TicketDataAccount,
			encodedTicketDataState.data
		);

		return decodedTicketDataState.data.charity_id.ticket_number_arr;
	});
	usersTicketNumberArr.forEach((numberArr, index) => {
		if (numberArr === winningNumberArr) {
			winnerUserTicketDataWalletsPK.push(ticketDataAccountPKArr[index]);
			winFlag = true;
		}
	});


	let encodedLotteryDataState = await connection.getAccountInfo(
		new PublicKey(lotteryDataAccountPK),
		"singleGossip"
	);
	let decodedLotteryDataState = borsh.deserialize(
		TicketDataSchema,
		TicketDataAccount,
		encodedLotteryDataState.data
	);
	let charityVC = [];
	charityVC.push(decodedLotteryDataState.data.is_lottery_initialised.charity_1_vc);
	charityVC.push(decodedLotteryDataState.data.is_lottery_initialised.charity_2_vc);
	charityVC.push(decodedLotteryDataState.data.is_lottery_initialised.charity_3_vc);
	charityVC.push(decodedLotteryDataState.data.is_lottery_initialised.charity_4_vc);
	let winningLotteryIndexes = [0];
	charityVC.forEach((value,index) => {
		if(index>0){
			if(value > charityVC[winningLotteryIndexes[0]]){
				winningLotteryIndex = [index];
				break;
			}
			else if (value === charityVC[winningLotteryIndexes[0]]){
				winningLotteryIndex.push(index);
				break;
			}
		}
	});
	let winningCharities = [];
	winningLotteryIndexes.forEach((winIndex) => {
		if(winIndex === 0){
			winningCharities.push(decodedLotteryDataState.data.is_lottery_initialised.charity_1_id);
		}
		if(winIndex === 1){
			winningCharities.push(decodedLotteryDataState.data.is_lottery_initialised.charity_2_id);
		}
		if(winIndex === 2){
			winningCharities.push(decodedLotteryDataState.data.is_lottery_initialised.charity_3_id);
		}
		if(winIndex === 3){
			winningCharities.push(decodedLotteryDataState.data.is_lottery_initialised.charity_4_id);
		}
	})

	if(winFlag === true){
		winnerUserTicketDataWalletsPK.forEach(async (publicKey) => {
			let encodedWinnerTicketDataState = await connection.getAccountInfo(
				new PublicKey(publicKey),
				"singleGossip"
			);
			let decodedWinnerTicketDataState = borsh.deserialize(
				TicketDataSchema,
				TicketDataAccount,
				encodedWinnerTicketDataState.data
			);
			winnerUserWalletsPK.push(decodedWinnerTicketDataState.data.charity_id.user_wallet_pk);
		});
		return {winningNumberArr,winnerUserWalletsPK,winningCharities}
	}
	else if(winFlag===false){
		return {winningNumberArr,winningCharities}
	}

	
};

module.exports = {
	lotteryDraw,
};
