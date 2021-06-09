const { model, Schema } = require('mongoose');

const ticketSchema = new Schema({
   walletID:  {type:  [Number], required:true}, //walletpk
   ticketArray: {type: [Number], required:true},
   charityId: Number,
   DataWallet: {type: [Number], required:true}, // DataAccountpubkey Id
   LotteryId:Number
});

module.exports = model('Tickets', ticketSchema);