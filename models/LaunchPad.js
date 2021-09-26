const { model, Schema } = require("mongoose");

const passLaunchSchema = new Schema({
  winnersWalletsId: {
    type: [String],
    required: true,
  },
  finishDate: {
    type: String,
    required: true,
  },
});

const launchSchema = new Schema({
  tokenName: { type: String, required: true },
  tokenLogo: { type: String, required: true },
  status: { type: Boolean, required: true, default: true },
  tokenAddress: { type: String, required: true },
  totalWinners: { type: Number, require: true },
  passLaunches: [passLaunchSchema],
  frequency: {
    type: Number,
    require: true,
  },
  dueDate: { type: String, require: true },
  endDate: { type: String, require: true },
  maxDeposit: { type: Number, require: true },
});

module.exports = model("Launch", launchSchema);
