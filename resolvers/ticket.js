const Ticket = require("../models/ticket");
const Lottery = require("../models/lottery");
module.exports = {
  Mutation: {
    async addTicket(
      _,
      { walletID, ticketArray, charityId, DataWallet, LotteryId },
      context,
      info
    ) {
      console.log({ walletID, ticketArray, charityId, DataWallet, LotteryId })
      const newTicket = new Ticket({
        walletID,
        ticketArray,
        charityId,
        DataWallet,
        LotteryId,
      });
      const lottery = await Lottery.findOne({ Id: LotteryId });
      const updateLottery = await Lottery.findOneAndUpdate(
        { Id: LotteryId, "CharityVoteCount.charityId": charityId },
        {
          $inc: {
            TotalRegistrations: 1,
            TotalPoolValue: lottery.TicketPrice,
            "CharityVoteCount.$.votes": 1,
          },
        },
        { new: true }
      );
      const res = await newTicket.save();
      return "Ticket Saved Successfully";
    },
  },
  Query: {
    async getDataWallets(_, args, context, info) {
      const tickets = await Ticket.find();
      return tickets;
    },
  },
};
