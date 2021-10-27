const { model, Schema } = require("mongoose");

const ticketSchema = new Schema({
  walletId: {
    type: String,
    required: true,
  },
  dataAccountId: {
    type: String,
    required: true,
  },
  transactionId: {
    type: String,
    required: true,
  },
});

const prizeSchema = new Schema({
  image: {
    type: String,
    required: true,
  },
  address: {
    type: String,
    required: true,
  },
  name: {
    type: String,
    required: true,
  },
  collectionName: {
    type: String,
    required: true,
  },
});

const nftSchema = new Schema({
  prizes: [prizeSchema],
  endDate: {
    type: String,
    required: true,
  },
  ticketPrice: {
    type: Number,
    required: true,
  },
  status: {
    type: String,
    required: true,
    enum: ["draft", "live", "completed"],
  },
  tickets: {
    type: [ticketSchema],
  },
});

module.exports = model("Nft", nftSchema);
