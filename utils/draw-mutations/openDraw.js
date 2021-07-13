const Lottery = require("../../models/Lottery");
const moment = require("moment");
const Drawing = require("../../models/Drawing");
// const { initLottery }  = require('../on-chain-instructions/initLottery')
const openDrawing = async (activeDrawing, charities) => {
  var newDraw = {};
  const lottery = await Lottery.findById(process.env.LOTTERY_ID);
  
  //60c447da624b8a3d5095baa8
  //60c9be158676ea0799255ee4
  var day = moment(activeDrawing.EndDate).format("dddd");
  var charityVote = [];
  var drawCharity = [];
  charities.map((t) => {
    charityVote.push({
      charityId: t.id,
      votes: 0,
    });
    drawCharity.push(t.id);
  });

  console.log(drawCharity);

  if (day === "Wednesday") {
    if (lottery.TotalPoolValue < 10) {
      newDraw = new Drawing({
        Charities: drawCharity,
        StartDate: activeDrawing.EndDate,
        EndDate: moment(activeDrawing.EndDate).add(3, "days").utc().format(),
        isActive: true,
        CharityVoteCount: charityVote,
        TotalPoolValue: 10,
      });
    } else {
      newDraw = new Drawing({
        Charities: drawCharity,
        StartDate: activeDrawing.EndDate,
        EndDate: moment(activeDrawing.EndDate).add(3, "days").utc().format(),
        isActive: true,
        CharityVoteCount: charityVote,
        TotalPoolValue: lottery.TotalPoolValue,
      });
    }
    await newDraw.save();
    console.log("draw open");
  } else if (day === "Saturday") {
    if (lottery.TotalPoolValue < 10) {
      newDraw = new Drawing({
        Charities: drawCharity,
        StartDate: activeDrawing.EndDate,
        EndDate: moment(activeDrawing.EndDate).add(4, "days").utc().format(),
        isActive: true,
        CharityVoteCount: charityVote,
        TotalPoolValue: 10,
      });
    } else {
      newDraw = new Drawing({
        Charities: drawCharity,
        StartDate: activeDrawing.EndDate,
        EndDate: moment(activeDrawing.EndDate).add(4, "days").utc().format(),
        isActive: true,
        CharityVoteCount: charityVote,
        TotalPoolValue: lottery.TotalPoolValue,
      });
    }

    await newDraw.save();
    console.log("draw open");
  } else {
    console.log("no new lot");
  }
  // const {lotteryDataSK, lotteryId} = await initLottery(charities);
  
  //storing the encrypted lottery data account in db in order to use it again

  // await Lottery.findByIdAndUpdate(process.env.LOTTERY_ID,{LotteryId:lotteryId,  LotteryDataAccount:lotteryDataSK})
};



module.exports = {
  openDrawing,
};
