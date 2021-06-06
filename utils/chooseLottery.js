
const { closePreviousLottery, startNextLottery } = require("./alterLottery");
exports.chooseLottery = async (a) => {
console.log("cron excecuted at", a);
  if (a === 1) {
    startNextLottery(a);
    a = a + 1;
    return a;
  } else {
    const lot = startNextLottery(a);
    if (lot === null) return (a = 1);
    closePreviousLottery(a - 1);

    a = a + 1;
    return a;
  }
};
