const moment = require("moment");
const Lottery = require("../models/Lottery");

module.exports = {
  Query: {
    async getLotteryInfo(parent, args, context, info){
        const lottery =await Lottery.find();

        return lottery[0];
    }
  }
};
