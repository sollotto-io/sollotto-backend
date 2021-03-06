const { model, Schema } = require("mongoose");

const raffleSchema = new Schema({
  raffleName: {
    type: String,
    require: true,
  },
  urlSlug: {
    type: String,
    require: true,
  },
  raffleImage: {
    type: String,
    require: true,
  },
  sollotoBranding: {
    type: Boolean,
    default: true,
  },
  testingWA: {
    type: String,
    require: true,
  },
  liveWA: {
    type: String,
    require: true,
  },
  operatorWa: {
    type: String,
    require: true,
  },
  vanityUrl: {
    type: String,
    require: true,
  },
  raffleStatus: {
    type: String,
    require: true,
  },
  Status: {
    type: Boolean,
    require: true,
    default: true,
  },
});

module.exports = model("Raffle", raffleSchema);
