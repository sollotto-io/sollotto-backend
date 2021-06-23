const Lottery = require("../models/Lottery");
const Drawing = require("../models/Drawing");
const Charity = require("../models/Charity");
const moment = require("moment");
var random = require("random");
const _ = require("lodash");
const { lotteryDraw } = require("./lotteryDraw");

const closeDrawing = async (drawing) => {
  var winningCharity = [];
  const result = await lotteryDraw(drawing);

  let max = 0;
  await drawing.CharityVoteCount.forEach(async (charity) => {
    if (charity.votes > max) {
      max = charity.votes;
      winningCharity = [charity.charityId.toString()];
    } else if (charity.votes === max) {
     winningCharity.push(charity.charityId.toString());
    } else {
      await Charity.findByIdAndUpdate(charity.charityId, {
        lifeTimeWins:0,
        currentVotes: 0,
      });
    }
  });

  winningCharity.forEach(async (charityId) => {
    await Charity.findByIdAndUpdate(charityId, {
      $inc: {
        lifeTimeWins: 1,
      },
      currentVotes: 0,
    });
  });
  if (result.winFlag === false) {
    await Lottery.findByIdAndUpdate(
      "60c9be158676ea0799255ee4",
      { $inc: { TotalPoolValue: drawing.TotalPoolValue * 0.65 } },
      { new: true }
    );

    await Drawing.findByIdAndUpdate(
      drawing._id,
      {
        isActive: false,
        $set: {
          WinningNumbers: result.winningNumberArr,
          WinningCharity: winningCharity,
        },
      },
      { new: true }
    );
  } else {
    await Lottery.findByIdAndUpdate(
      "60c9be158676ea0799255ee4",
      { TotalPoolValue: 0 },
      { new: true }
    );

    await Drawing.findByIdAndUpdate(
      drawing._id,
      {
        isActive: false,
        $set: {
          WinningNumbers: result.winningNumberArr,
          WinningCharity: winningCharity,
          WinnerWallet: result.winnerUserWalletsPK,
        },
      },
      { new: true }
    );
  }
  console.log("draw closed");
};
exports.changeDraw = async () => {
  const activeDrawing = await Drawing.findOne({ isActive: true })
    .populate("Tickets")
    .exec();

  if (activeDrawing) {
    await closeDrawing(activeDrawing);
    await openDrawing(activeDrawing);
  } else {
    console.log("no active");
  }
};

const openDrawing = async (activeDrawing) => {
  const lottery = await Lottery.findById("60c9be158676ea0799255ee4");
  var day = moment(activeDrawing.EndDate).format("dddd");
  
  const charityVote = [];
  activeDrawing.Charities.map((t) => {
    charityVote.push({
      charityId: t,
      votes: 0,
    });
  });
  if (day === "Wednesday") {
    const newDraw = new Drawing({
      Charities: activeDrawing.Charities,
      StartDate: activeDrawing.EndDate,
      EndDate: moment(activeDrawing.EndDate).add(3, "days").utc().format(),
      isActive: true,
      CharityVoteCount: charityVote,
      TotalPoolValue: lottery.TotalPoolValue,
    });
    await newDraw.save();
  } else if (day === "Saturday") {
    const newDraw = new Drawing({
      Charities: activeDrawing.Charities,
      StartDate: activeDrawing.EndDate,
      EndDate: moment(activeDrawing.EndDate).add(4, "days").utc().format(),
      isActive: true,
      CharityVoteCount: charityVote,
      TotalPoolValue: lottery.TotalPoolValue,
    })
    await newDraw.save();
  }else{
    console.log("no new lot")
  }
  console.log("draw open")
};
