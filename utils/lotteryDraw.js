const {
	SystemProgram,
	PublicKey,
	Transaction,
	TransactionInstruction,
	Account,
	SYSVAR_RENT_PUBKEY,
	LAMPORTS_PER_SOL,
	Connection
} = require("@solana/web3.js");
var borsh = require("borsh");
const { TicketDataAccount,TicketDataSchema } = require("./TicketDataBorsh.js");
const { LotteryDataAccount, LotteryDataSchema } = require("./LotteryDataBorsh.js");
var random = require("random");

const lotteryDraw = async (data) => {
	
	let connection = new Connection("https://devnet.solana.com");
	let holdingWalletAccount = new Account(
		Buffer.from([
			143, 209, 242, 241, 76, 148, 73, 213, 127, 35, 252, 134, 149, 170, 105,
			228, 176, 172, 85, 112, 147, 193, 165, 221, 82, 188, 85, 12, 190, 244,
			177, 149, 105, 128, 153, 47, 218, 83, 112, 164, 53, 80, 41, 154, 162, 143,
			160, 198, 132, 145, 53, 112, 105, 82, 79, 229, 179, 120, 219, 61, 27, 12,
			203, 59,
		])
	);
	let winnerUserLotteryDataWalletsPK = [];
	let lotteryId = null;
	let lotterDataAccountPK = null;
	let totalPool = null;
	let lotteryDataAccountPKArr = [];
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
	let usersTicketNumberArr = lotteryDataAccountPKArr.map(async (publicKey) => {
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
			winnerUserLotteryDataWalletsPK.push(lotteryDataAccountPKArr[index]);
			winFlag = true;
		}
	});
	winnerUserLotteryDataWalletsPK.forEach( async(publicKey) => {
		let encodedWinnerTicketDataState = await connection.getAccountInfo(
			new PublicKey(publicKey),
			"singleGossip"
		);
		let decodedWinnerTicketDataState = borsh.deserialize(
			TicketDataSchema,
			TicketDataAccount,
			encodedWinnerTicketDataState.data
		);
		winnerUserWalletsPK.push(decodedWinnerTicketDataState.data.user_wallet_pk);
	});

	//Lottery Win Distribution
	let totalWinners = winnerUserWalletsPK.length;
	let userWinAmount = totalPool / totalWinners;
	let solTransferTx = [];
	winnerUserWalletsPK.forEach((publicKey) => {
		solTransferTx.push(
			SystemProgram.transfer({
				fromPubkey: holdingWalletAccount.publicKey,
				toPubkey: publicKey,
				lamports: userWinAmount * LAMPORTS_PER_SOL,
			})
		);
	});
	let transaction = new Transaction().add(solTransferTx);
	// let signers = [lotteryDataAccount];
	transaction.recentBlockhash = (
		await connection.getRecentBlockhash()
	).blockhash;
	transaction.setSigners(holdingWalletAccount.publicKey);
	// if (signers.length > 0) {
	// 	transaction.partialSign(...signers);
	// }
	let signedTx = await holdingWalletAccount.signTransaction(
		transaction
	);
	let signature = await connection.sendRawTransaction(signedTx.serialize());

	await connection.confirmTransaction(signature, "singleGossip");

	console.log(
		`Winning Numbers: ${winningNumberArr} \nWinner Wallet: ${
			winnerUserWalletPK ? winnerUserWalletPK : "None"
		} \n`
	);
};

module.exports = {
	lotteryDraw
}