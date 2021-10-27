const Pool = require("../../models/Pool");
const verifyPool = async () => {
  console.log("verifying pool");
  /* const pools = await Pool.find().sort({ createdAt: -1 });

  if (pools) {
    pools.forEach(async (pool) => {
      if (new Date(Date.now()) > new Date(pool.endDate)) {
        const currentPool = await Pool.findById(pool.id);
        if (currentPool) {
          const endDate = new Date(pool.endDate);
          endDate.setDate(endDate.getDate() + pool.frequency);
          currentPool.endDate = endDate.toDateString() + " GMT-8";

          currentPool.passPools.push({
            winningWalletId: "Test",
            finishDate: pool.endDate,
          });
        }
        await currentPool.save();
      }
    });
  } */
};

module.exports = verifyPool;
