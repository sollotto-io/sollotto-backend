const moment = require("moment");
const LaunchPad = require("../models/LaunchPad");

module.exports = {
  Query: {
    async getAllLaunched(_, { UserPK }, context, info) {
      try {
        const LaunchPads = await LaunchPad.find().sort({ createdAt: -1 });
        return LaunchPads;
      } catch (err) {
        throw new Error(err);
      }
    },
    async getLaunchPadById(_, { Id }, context, info) {
      try {
        const res = await LaunchPad.findById(Id);
        return res;
      } catch (err) {
        throw new Error(err);
      }
    },
  },
  Mutation: {
    async AddLaunchPad(
      _,
      {
        LaunchPadInput: {
          PoolName,
          PoolImage,
          Status,
          TotalWinners,
          TimeRemaining,
          MaxDeposit,
        },
      },
      context,
      info
    ) {
      const newLaunch = new LaunchPad({
        PoolName,
        PoolImage,
        Status,
        TotalWinners,
        TimeRemaining,
        MaxDeposit,
      });

      await newLaunch.save();
      return "Launch Pad Lottery is saved";
    },
    async changeLaunchState(_, {Id, Status}, context, info) {
      await LaunchPad.findByIdAndUpdate(Id, { Status: Status });
  
      return "Charity Status Updated";
    },
  },
  
};
