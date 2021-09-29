const { model, Schema } = require("mongoose");

const passModel4 = new Schema({
  winningWalletId: {
    require: true,
    type: String,
  },
  finishDate: {
    require: true,
    type: String,
  },
});

const model4Schema = new Schema({
  endDate: {
    type: String,
    require: true,
  },
  passModel4: [passModel4],
});

module.exports = model("Model4", model4Schema);
