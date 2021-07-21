const { model, Schema } = require("mongoose");

const drawingSchema = new Schema({
  StartDate: String,
  EndDate: String,
  WinningCharity: [{ type: Schema.Types.ObjectId, ref: "Charity" }],
  WinnerWallet: { type: [[Number]] },
  Charities: [{ type: Schema.Types.ObjectId, ref: "Charity" }],
  CharityVoteCount: [
    {
      charityId: { type: Schema.Types.ObjectId, ref: "Charity" },
      votes: { type: Schema.Types.Number },
    },
  ],
  WinningNumbers: { type: [Number] },
  Tickets: [{ type: Schema.Types.ObjectId, ref: "Ticket" }],
  isActive: { type: Schema.Types.Boolean },
  TotalRegistrations: { type: Number },
  TotalPoolValue:{type: Number},
  TransactionId:{type:String}
});

module.exports = model("Drawing", drawingSchema);
