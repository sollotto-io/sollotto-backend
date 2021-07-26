const { model, Schema } = require('mongoose');

const raffleSchema = new Schema({
  raffleName: String,
  publicKey : String,
  Status: Boolean,
  ImageURL:String,
});

module.exports = model('Raffle', raffleSchema);
