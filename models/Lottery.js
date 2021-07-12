const { model, Schema, Types } = require("mongoose");

const lotterySchema = new Schema({
  LotteryId: {type:Schema.Types.Decimal128},
  TicketPrice: { type: Number },
  TotalPoolValue: { type: Number },
  LotteryDataAccount: { type: String },
});

module.exports = model("Lottery", lotterySchema);
