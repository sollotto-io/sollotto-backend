const { model, Schema, Types } = require("mongoose");

const lotterySchema = new Schema({
  TicketPrice: { type: Number },
  TotalPoolValue: { type: Number },
  LotteryDataAccount: { type: [Number] },
});

module.exports = model("Lottery", lotterySchema);
