const Model4 = require("../../models/Model4");

const verifyModel4 = async () => {
  console.log("verifying model 4");
  /*   const model4 = await Model4.find();

  const currentModel4 = await Model4.findById(model4[0]._id);

  if (new Date(Date.now()) > new Date(currentModel4.endDate)) {
    currentModel4.passModel4.push({
      winningWalletId: "DR9bNjsv25meGDeXqQLqf6Xoo1LBpdhP6wiuQ4ir2Jmo",
      finishDate: currentModel4.endDate,
    });
    const endDate = new Date(currentModel4.endDate);
    endDate.setMonth(endDate.getMonth() + 1);
    currentModel4.endDate = endDate.toDateString() + " GMT-8";
    await currentModel4.save();
  } */
};

module.exports = verifyModel4;
