const { model, Schema } = require("mongoose");

const passPoolSchema = new Schema({
  winningWalletId: {
    require: true,
    type: String,
  },
  finishDate: {
    require: true,
    type: String,
  },
});

const poolSchema = new Schema({
  tokenName: {
    type: String,
    require: true,
  },
  tokenLogo: {
    type: String,
    require: true,
  },
  dueDate: {
    type: String,
    require: true,
  },
  passPools: [passPoolSchema],
  tokenAddress: {
    type: String,
    require: true,
  },
  frequency: {
    type: Number,
    require: true,
  },
  endDate: {
    type: String,
    required: true,
  },
  status: {
    type: Boolean,
    require: true,
    default: true,
  },
});

module.exports = model("Pool", poolSchema);
