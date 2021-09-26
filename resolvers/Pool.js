const { addCatchUndefinedToSchema } = require("graphql-tools");
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
        const { tokenName, tokenLogo, dueDate, tokenAddress, frequency } =
          poolInput;

        const endDate = new Date(dueDate);
        endDate.setDate(endDate.getDate() + frequency);
        const newPool = new Pool({
          tokenName,
          tokenLogo,
          dueDate,
          tokenAddress,
          frequency,
          endDate: endDate.toDateString() + " GMT-8",
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
      const { tokenName, tokenLogo, dueDate, tokenAddress, frequency } =
        poolInput;

      try {
        let freq;
        const pool = await Pool.findById(poolId);
        pool.tokenName = tokenName;
        pool.tokenLogo = tokenLogo;
        pool.tokenAddress = tokenAddress;

        if (pool.frequency !== frequency) {
          freq = frequency;
          const endDate = new Date(pool.endDate);
          endDate.setDate(endDate.getDate() + (frequency - pool.frequency));
          pool.endDate = endDate.toDateString() + " GMT-8";
        } else {
          freq = pool.frequency;
        }
        pool.frequency = frequency;

        if (pool.dueDate !== dueDate) {
          const endDate = new Date(dueDate);
          endDate.setDate(endDate.getDate() + freq);
          pool.endDate = endDate.toDateString() + " GMT-8";
        }
        pool.dueDate = dueDate;

        await pool.save();
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

  async resetPool(_, { poolId }, context, info) {
    const pool = await Pool.findById(poolId);

    if (Date.now() > new Date(pool.endDate)) {
      console.log("AHHHHHH");
    }
  },
};

module.exports = {
  Query: poolResolvers.Query,

  Mutations: protectedResolvers(poolResolvers.Mutations),
};
