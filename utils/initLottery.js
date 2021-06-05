import {
	SystemProgram,
	PublicKey,
	Transaction,
	TransactionInstruction,
	Account,
	SYSVAR_RENT_PUBKEY,
} from "@solana/web3.js";
import * as borsh from "borsh";
import {
	IncomingLotteryDataAccount,
	LotteryDataAccount,
	IncomingLotteryDataSchema,
	LotteryDataSchema,
} from "./LotteryDataBorsh";

export const initLottery = async (lotteryData) => {
	let connection = new Connection(process.env.SOLANA_NETWORK);

	let holdingWalletAccount = new Account(
		Buffer.from([
			143, 209, 242, 241, 76, 148, 73, 213, 127, 35, 252, 134, 149, 170, 105,
			228, 176, 172, 85, 112, 147, 193, 165, 221, 82, 188, 85, 12, 190, 244,
			177, 149, 105, 128, 153, 47, 218, 83, 112, 164, 53, 80, 41, 154, 162, 143,
			160, 198, 132, 145, 53, 112, 105, 82, 79, 229, 179, 120, 219, 61, 27, 12,
			203, 59,
		])
	);
	let solanaProgramId = new PublicKey(process.env.SOLANA_INIT_LOTTERY_PROGRAM);

	try {
		const lotteryDataAccount = new Account();
		const createLotteryDataAccountTx = SystemProgram.createAccount({
			space: 81,
			lamports: await connection.getMinimumBalanceForRentExemption(
				81,
				"singleGossip"
			),
			fromPubkey: holdingWalletAccount.publicKey,
			newAccountPubkey: lotteryDataAccount.publicKey,
			programId: solanaProgramId,
		});

		const value = new IncomingLotteryDataAccount(
			lotteryData.lotteryId,
			lotteryData.charities[0],
			lotteryData.charities[1],
			lotteryData.charities[2],
			lotteryData.charities[3],
			lotteryData.ticketPrice
		);
		const buffer = borsh.serialize(IncomingLotteryDataSchema, value);
		const dataArr = new Uint8Array([0, ...buffer]);

		const initLotteryTx = new TransactionInstruction({
			programId: solanaProgramId,
			keys: [
				{
					pubkey: lotteryDataAccount.publicKey,
					isSigner: false,
					isWritable: true,
				},
				{ pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
				{
					pubkey: holdingWalletAccount.publicKey,
					isSigner: true,
					isWritable: false,
				},
			],
			data: dataArr,
		});
		let transaction = new Transaction().add(
			createLotteryDataAccountTx,
			initLotteryTx
		);
		let signers = [lotteryDataAccount];
		transaction.recentBlockhash = (
			await connection.getRecentBlockhash()
		).blockhash;
		transaction.setSigners(
			holdingWalletAccount.publicKey,
			...signers.map((s) => s.publicKey)
		);
		if (signers.length > 0) {
			transaction.partialSign(...signers);
		}
		let signedTx = await holdingWalletAccount.publicKey.signTransaction(
			transaction
		);
		let signature = await connection.sendRawTransaction(signedTx.serialize());
		console.log(
			"Submitted transaction " + signature + ", awaiting confirmation"
		);
		await connection.confirmTransaction(signature, "singleGossip");
		const encodedLotteryState = (
			await connection.getAccountInfo(
				lotteryDataAccount.publicKey,
				"singleGossip"
			)
		).data;
		const decodedLotteryState = borsh.deserialize(
			LotteryDataSchema,
			LotteryDataAccount,
			encodedLotteryState
		);

		console.log(`Lottery Data: ${JSON.stringify(decodedLotteryState)}`);
		console.log(
			`Lottery Data Account PK: ${lotteryDataAccount.publicKey.toBase58()}`
		);
		console.log("Transaction " + signature + " confirmed");
	} catch (e) {
		console.warn(e);
		console.log("Error: " + e.message);
	}
};
