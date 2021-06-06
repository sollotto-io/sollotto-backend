const PoolModel = require("../models/pool");
const { CHARITY_STATUS } = require("../config");
module.exports = {
  Mutation: {
    async addPool(
      _,
      {
        poolInput: {
          PoolName,
          Pool,
          PrizePool,
          TimeRemaining,
          PoolARP,
          TotalDeposit,
          TotalLiquidity,
          Odds,
          currentTicketPrice
        },
      },
      context,
      info
    ) {
      const newPool = new PoolModel({
        PoolName,
        Pool,
        PrizePool,
        TimeRemaining,
        PoolARP,
        TotalDeposit,
        TotalLiquidity,
        Odds,
        currentTicketPrice
      });
      const res = await newPool.save();
      return {
        ...res._doc,
        id: res._id,
      };
    },
  },
  Query: {
    async getAllPools(_, args, context, info) {
      try {
        const pools = await PoolModel.find().sort({ createdAt: -1 });
        return pools;
      } catch (err) {
        throw new Error(err);
      }
    },
    // async getActiveCharities(_, args, context, info) {
    //   try {
    //     const charities = await Charity.find().sort({ createdAt: -1 });
    //     activeCharities = charities.filter(p=>p.Status === CHARITY_STATUS.VOTE_NOW)
    //     return activeCharities;
    //   } catch (err) {
    //     throw new Error(err);
    //   }
    // },
  },
};
