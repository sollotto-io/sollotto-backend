const moment = require("moment");
const Lottery = require("../models/lottery");

module.exports = {
  Mutations: {
    async addLottery(
      _,
      { LotteryInput: { Id, Charities, TicketPrice,StartDate, EndDate  } },
      context,
      info
    ) {
      const votecount = [];
      Charities.map((c) => votecount.push({ charityId: c, votes: 0 }));
      const newLottery = new Lottery({
        Id,
        Charities,
        TicketPrice,
        StartDate,
        EndDate,
        WinnerWallet: [],
        TotalPoolValue: 0,
        TotalRegistrations: 0,
        isActive: false,
        LotteryDataAccount: [],
        CharityVoteCount: votecount,
      });
      console.log(newLottery)
      await newLottery.save();
      return "Lottery Added Succesfully";
    },
  },
  Query: {
    async getupcomingLottery(_, args, context, info) {
      const Lotteries = await Lottery.find();
      const upcomingLottery = Lotteries.find(
        (l) => moment(l.StartDate).format("L") === moment().format("L")
      );
      return upcomingLottery;
    },
  },
};
