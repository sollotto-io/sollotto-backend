const moment = require("moment");
const Lottery = require("../models/Lottery");

module.exports = {
  Mutations: {
      async addLottery(_,{LotteryInput: {LotteryDataAccount}}, context, args){
        const newLottery = new Lottery({
          LotteryDataAccount
        })

        await newLottery.save()
        return "Lottery added successful"
      }
  },
  Query: {
  }
};
