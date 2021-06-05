const { model, Schema } = require("mongoose");

const lotterySchema = new Schema({
  Id: Number,
  Charities:{type:[Number]},
  TicketPrice: {type:Number},
  StartDate: String,
  EndDate: String,
  WinnerWallet: {type:[Number]},
  TotalPoolValue: {type: Number},
  TotalRegistrations: {type: Number},
  OnGoing: {type:Schema.Types.Boolean},
  LotteryDataAccount:{type:[Number]},
  isActive:{type:Schema.Types.Boolean},
  CharityVoteCount:[{
      charityId:{type:Schema.Types.Number},
      votes:{type:Schema.Types.Number}
  }]
});

module.exports = model("Lottery", lotterySchema);
