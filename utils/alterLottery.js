const Lottery = require("../models/lottery");
const { lotteryDraw } = require("./lotteryDraw");

const startNextLottery = async (i) => {
	const lot = await Lottery.findOneAndUpdate(
		{ Id: i },
		{ isActive: true },
		{ new: true }
	);

	return lot;
};

const setDataBase = async (i, winningCharities, winningNumberArr) => {
	await Lottery.findOneAndUpdate(
		{ Id: i },
		{
			WinningCharity: winningCharities,
			WinningNumbers: winningNumberArr,
		}
	);
};

const setWinningUsers = async (
	i,
	winningCharities,
	winningNumberArr,
	winnerUserWalletsPK
) => {
	await Lottery.findOneAndUpdate(
		{ Id: i },
		{
			WinningCharity: winningCharities,
			WinningNumbers: winningNumberArr,
			WinnerWallet: winnerUserWalletsPK,
		}
	);
};

const closePreviousLottery = async (i) => {
	const lottoremove = await Lottery.findOneAndUpdate(
		{ Id: i },
		{isActive:false},
		{new:true}
	);
	const drawData = lotteryDraw(lottoremove);
	
	drawData.then((d) => {
		console.log(d)
		if (d.winFlag === false) {
			setDataBase(i, d.winningCharities, d.winningNumberArr);
		} else {
			setWinningUsers(
				i,
				d.winningCharities,
				d.winnerUserWalletsPK,
				d.winningNumberArr
			);
		}
	});
};

module.exports = {
	startNextLottery,
	closePreviousLottery,
};
