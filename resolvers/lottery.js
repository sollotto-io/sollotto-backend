const moment = require("moment");
const Lottery = require("../models/lottery");

module.exports = {
  Mutations: {
    async addLottery(
      _,
      { LotteryInput: { Id, Charities, TicketPrice, StartDate, EndDate } },
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
        WinningNumbers:[],
        WinningCharity:[],
        WinnerWallet: [],
        TotalPoolValue: 0,
        TotalRegistrations: 0,
        isActive: false,
        LotteryDataAccount: [
          107, 137, 21, 229, 174, 33, 93, 169, 125, 138, 22, 103, 244, 240, 45,
          120, 136, 154, 66, 111, 9, 128, 23, 194, 96, 144, 119, 247, 131, 169,
          138, 100,
        ],
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
      console.log(Id);
      const lottery = await Lottery.findOne({ Id: Id });

      return lottery;
    },
  },
};
