const { PublicKey, Connection } = require("@solana/web3.js");
var borsh = require("borsh");
const sleep = require("util").promisify(setTimeout);
const { TicketDataAccount, TicketDataSchema } = require("./TicketDataBorsh.js");
const _ = require("lodash");
var random = require("random");
const dotenv = require("dotenv");
dotenv.config();
const { sortTicketNumber } = require("./helpers");

const lotteryDraw = async (data) => {
	var lotteryDataAcc = [];
	await data.Tickets.map((t) => {
		lotteryDataAcc.push(t.DataWallet);
	});

	let connection = new Connection(process.env.SOLANA_NETWORK);
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
	winningNumberArr = sortTicketNumber(winningNumberArr);

	let winFlag = false;
	const waitFor = (ms) => new Promise((r) => setTimeout(r, ms));

	async function asyncForEach(array, callback) {
		for (let index = 0; index < array.length; index++) {
			await callback(array[index], index, array);
		}
	}

	const start = async () => {
		await asyncForEach(ticketDataAccountPKArr, async (publicKey, i) => {
			await waitFor(300);

			const encodedTicketDataState = await connection.getAccountInfo(
				new PublicKey(publicKey),
				"singleGossip"
			);
			const decodedTicketDataState = await borsh.deserialize(
				TicketDataSchema,
				TicketDataAccount,
				encodedTicketDataState.data
			);

			if (
				_.isEqual(
					sortTicketNumber(
						Buffer.from(
							decodedTicketDataState.charity_id.ticket_number_arr
						).toJSON().data
					),
					winningNumberArr
				)
			) {
				await winnerUserTicketDataWalletsPK.push(ticketDataAccountPKArr[i]);
				winFlag = true;
			}
		});
	};

	await start();
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

	const start2 = async () => {
		await asyncForEach(winnerUserTicketDataWalletsPK, async (publicKey, i) => {
			let encodedWinnerTicketDataState = await connection.getAccountInfo(
				new PublicKey(publicKey),
				"singleGossip"
			);

			let decodedWinnerTicketDataState = await borsh.deserialize(
				TicketDataSchema,
				TicketDataAccount,
				encodedWinnerTicketDataState.data
			);
			await winnerUserWalletsPK.push(
				Buffer.from(
					decodedWinnerTicketDataState.charity_id.user_wallet_pk
				).toJSON().data
			);
		});
	};

	if (winFlag === true) {
		await start2();
		return { winFlag, winningNumberArr, winnerUserWalletsPK };
	} else {
		return { winFlag, winningNumberArr };
	}
};

module.exports = {
	lotteryDraw,
};
