const Lottery = require("../models/Lottery");
const Drawing = require("../models/Drawing");
const moment = require("moment");
var random = require("random");
const _  = require('lodash');
const {lotteryDraw}  = require('./lotteryDraw')


const closeDrawing = async (drawing) => {

  var winningCharity = [];
  const result = await lotteryDraw(drawing)
 
  let max = 0;
  await drawing.CharityVoteCount.forEach((charity) => {
    if (charity.votes > max) {
      max = charity.votes;
      winningCharity = [charity.charityId.toString()];
    } else if (charity.votes === max) {
      winningCharity.push(charity.charityId.toString());
    }
  });

  if (result.winFlag === false) {
    await Lottery.findByIdAndUpdate(
      "60c447da624b8a3d5095baa8",
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
  }
  else{
    await Lottery.findByIdAndUpdate(
      "60c447da624b8a3d5095baa8",
      {  TotalPoolValue: 0 },
      { new: true }
    );
   
    await Drawing.findByIdAndUpdate(
      drawing._id,
      {
        isActive: false,
        $set: {
          WinningNumbers: result.winningNumberArr,
          WinningCharity: winningCharity,
          WinnerWallet:result.winnerUserWalletsPK
        },
      },
      { new: true }
    );

  }

};
exports.changeDraw = async () => {
  const activeDrawing = await Drawing.findOne({isActive:true}).populate('Tickets').exec();
  
  if (activeDrawing) {
    await closeDrawing(activeDrawing);
    await openDrawing(activeDrawing);
  } else {
   console.log("no active")
  }
};

const openDrawing = async (activeDrawing) => {
  const lottery = await Lottery.findById("60c447da624b8a3d5095baa8")
  console.log(lottery)
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
      EndDate: moment().day(6).utc().format(),
      isActive: true,
      CharityVoteCount: charityVote,
      TotalPoolValue:lottery.TotalPoolValue
    });
    await newDraw.save();
  
  } else if (day === "Saturday") {
   
    const newDraw = new Drawing({
      Charities: activeDrawing.Charities,
      StartDate: activeDrawing.EndDate,
      EndDate: moment().day(10).utc().format(),
      isActive: true,
      CharityVoteCount: charityVote,
      TotalPoolValue:lottery.TotalPoolValue

    });
   await newDraw.save();
    
  }
};
