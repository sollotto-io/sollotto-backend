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
  dueDate: {
    type: Date,
    require: true,
  },
  tokenAddress: {
    type: String,
    require: true,
  },
  status: {
    type: Boolean,
    require: true,
    default: true,
  },
});

module.exports = model("Pool", poolSchema);
