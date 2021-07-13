const Charity = require("../../models/Charity");
const Drawing = require("../../models/Drawing");
const Lottery = require("../../models/Lottery");
const { lotteryDraw } = require("../lotteryDraw");
const {
  storeWinningNumbers,
} = require("../on-chain-instructions/storeWinningNumbers");

const {rewardWinner} = require('../on-chain-instructions/rewardWinner')

const closeDrawing = async (drawing) => {
  var winningCharity = [];
  const result = await lotteryDraw(drawing);
  const lottery = await Lottery.findById(process.env.LOTTERY_ID);

  // storewinner number instruction call
  //    await storeWinningNumbers(
  //   lottery.LotteryDataAccount,
  //   result.winningNumberArr
  // );
  // // reward winner instruction call
  
  //  await rewardWinner(lottery.LotteryId, lottery.LotteryDataAccount,drawing.Charities,result.winningNumberArr);

  let max = 0;
  await drawing.CharityVoteCount.forEach(async (charity) => {
    if (charity.votes > max) {
      max = charity.votes;
      winningCharity = [charity.charityId.toString()];
    } else if (charity.votes === max) {
      winningCharity.push(charity.charityId.toString());
    } else {
      await Charity.findByIdAndUpdate(charity.charityId, {
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
      "60c447da624b8a3d5095baa8",
      { $inc: { TotalPoolValue: drawing.TotalPoolValue * 0.65 } },
      { new: true }
    );
    //60c447da624b8a3d5095baa8
    //60c9be158676ea0799255ee4
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
      "60c447da624b8a3d5095baa8",
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

module.exports = {
  closeDrawing,
};
