const Pool = require("../models/Pool");

module.exports = {
  Query: {
    async getAllPools() {
      try {
        const pools = await Pool.find().sort({ createdAt: -1 });
        return pools;
      } catch (e) {
        return e;
      }
    },
    async getSinglePool(_, { poolId }, context, info) {
      try {
        const pool = await Pool.findById(poolId);
        return pool;
      } catch (e) {
        return e;
      }
    },
  },

  Mutations: {
    async addPool(_, { poolInput }, context, info) {
      try {
        const {
          tokenName,
          tokenLogo,
          prizePool,
          DueDate,
          tokenAddress,
          depositLimit,
          numberOfWinners,
        } = poolInput;

        const newPool = new Pool({
          tokenName,
          tokenLogo,
          prizePool,
          DueDate,
          tokenAddress,
          depositLimit,
          numberOfWinners,
        });
        const pool = await newPool.save();
        if (pool) {
          return pool;
        } else {
          return `Couldn't create pool error`;
        }
      } catch (e) {
        return e;
      }
    },
    async updatePool(_, { poolId, poolInput }, context, info) {
      const {
        tokenName,
        tokenLogo,
        prizePool,
        DueDate,
        tokenAddress,
        depositLimit,
        numberOfWinners,
      } = poolInput;

      try {
        const pool = await Pool.findByIdAndUpdate(
          poolId,
          {
            tokenName,
            tokenLogo,
            prizePool,
            DueDate,
            tokenAddress,
            depositLimit,
            numberOfWinners,
          },
          { new: true }
        );
        return pool;
      } catch (e) {
        return e;
      }
    },

    async changePoolStatus(_, { poolId, status }, context, info) {
      try {
        const pool = await Pool.findByIdAndUpdate(
          poolId,
          {
            status,
          },
          { new: true }
        );
        return pool;
      } catch (e) {
        return e;
      }
    },
  },
};
