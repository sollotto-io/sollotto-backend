const Lottery = require("../models/lottery");
const { initLottery } = require("./initLottery");
const { lotteryDraw } = require("./lotteryDraw");

const startNextLottery = async (i) => {
	const lot = await Lottery.findOneAndUpdate(
		{ Id: i },
		{ isActive: true },
		{ new: true }
	);
	console.log(lot);
	let lotteryData = {
		lotteryId: lot.Id,
		charities: lot.Charities,
		ticketPrice: lot.TicketPrice,
	};
	// initLottery(lotteryData)
	return lot;
};

const closePreviousLottery = async (i) => {
	const lottoremove = await Lottery.findOneAndUpdate(
		{ Id: i },
		{ isActive: false },
		{ new: true }
	);
	console.log(lottoremove);
	lotteryDraw();
};

module.exports = {
	startNextLottery,
	closePreviousLottery,
};