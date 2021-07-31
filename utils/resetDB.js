const Charity = require("../models/Charity");
const Drawing = require("../models/Drawing");
const Lottery = require("../models/Lottery");
const Ticket = require("../models/Ticket");
const moment = require("moment");
// const {initLottery} = require("./on-chain-instructions/initLottery")

const resetDb = async () => {
  console.log("clearing database....");
  //   await Drawing.deleteMany({});
  //   await Lottery.updateMany({},{LotteryId:null,TotalPoolValue:0,LotteryDataAccount:null})
  //   await Charity.updateMany({},{currentVotes:0,lifeTimeVotes:0,lifeTimeWins:0,nominationVotes:0,lifeTimeNominationVotes:0,LastNominationVote:null,})

  // console.log("starting fresh draw...");
  // const newDrawing = new Drawing({
  //   Charities: ["60c33e4adc27fe0bd818e59a"],
  //   StartDate: moment().utc().format(),
  //   EndDate: moment().add(3,'days').utc().format(),
  //   isActive:false,
  //   CharityVoteCount: {charityId:"60c33e4adc27fe0bd818e59a",votes:0},
  // });
  // const res = await newDrawing.save().then(d=>d.populate('Charities').execPopulate());
  // const {lotteryDataSK,lotteryId} = initLottery(res.Charities);

  //update the lotterydata in db
};

// const initLottery = (doc)=>{
//     console.log(typeof doc)
// }
module.exports = {
  resetDb,
};
