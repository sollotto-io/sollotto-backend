const Lottery = require("../models/lottery");

exports.chooseLottery = async (a) => {
  console.log("cron excecuted at", a);
  if (a === 1) {
    const lot = await Lottery.findOneAndUpdate(
      { Id: a },
      { isActive: true },
      { new: true }
    );
    console.log(lot);

    a = a + 1;
    return a;
  } else {
    const lot = await Lottery.findOneAndUpdate(
      { Id: a },
      { isActive: true },
      { new: true }
    );
    if (lot === null) return (a = 1);
    console.log(lot);
    const lottoremove = await Lottery.findOneAndUpdate(
      { Id: a - 1 },
      { isActive: false },
      { new: true }
    );
    console.log(lottoremove);
    a = a + 1;
    return a;
  }
};
