const moment = require("moment");
const Lottery = require("../models/lottery");

module.exports = {
  Mutations: {
    async addLottery(
      _,
      { LotteryInput: { Id, Charities, TicketPrice, StartDate, EndDate,LotteryDataAccount } },
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
        LotteryDataAccount,
        WinningNumbers:[],
        WinningCharity:[],
        WinnerWallet: [],
        TotalPoolValue: 0,
        TotalRegistrations: 0,
        isActive: false,
        CharityVoteCount: votecount,
        WinningCharityName: "",
      });
      console.log(newLottery);
      await newLottery.save();
      return "Lottery Added Succesfully";
    },
  },
  Query: {
    async getupcomingLottery(_, args, context, info) {
      const Lotteries = await Lottery.find();
      const upcomingLottery = Lotteries.find((l) => l.isActive === true);
      return upcomingLottery;
    },
    async getAllLotteries(_, args, context, info) {
      const Lotteries = await Lottery.find();
      return Lotteries;
    },

    async getLotteryById(_, { Id }, context, info) {
    
      const lottery = await Lottery.findOne({ Id: Id });

      return lottery;
    },
  },
};
