const { model, Schema } = require("mongoose");

const poolSchema = new Schema({
  tokenName: {
    type: String,
    require: true,
  },
  tokenLogo: {
    type: String,
    require: true,
  },
  prizePool: {
    type: Number,
    require: true,
  },
  DueDate: {
    type: Date,
    require: true,
  },
  tokenAddress: {
    type: String,
    require: true,
  },
  depositLimit: {
    type: Number,
    require: true,
  },
  numberOfWinners: {
    type: Number,
    require: true,
  },
  status: {
    type: Boolean,
    require: true,
    default: true,
  },
});

module.exports = model("Pool", poolSchema);
