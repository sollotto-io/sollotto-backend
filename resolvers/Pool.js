const Pool = require("../models/Pool");
const protectedResolvers = require("./utils");

const poolResolvers = {
  Query: {
    async getAllPools(_, params, context, info) {
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
        const { tokenName, tokenLogo, dueDate, tokenAddress } = poolInput;

        const newPool = new Pool({
          tokenName,
          tokenLogo,
          dueDate,
          tokenAddress,
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
      const { tokenName, tokenLogo, dueDate, tokenAddress } = poolInput;

      console.log(tokenName, tokenLogo, dueDate, tokenAddress);
      try {
        const pool = await Pool.findByIdAndUpdate(
          poolId,
          {
            tokenName,
            tokenLogo,
            dueDate,
            tokenAddress,
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

module.exports = {
  Query: poolResolvers.Query,

  Mutations: protectedResolvers(poolResolvers.Mutations),
};
