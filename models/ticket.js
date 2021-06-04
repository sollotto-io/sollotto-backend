const { model, Schema } = require('mongoose');

const ticketSchema = new Schema({
   walletID:  {type:  [Number], required:true}, //walletpk
   ticketArray: {type: [String], required:true},
   charityId: Number,
   DataWallet: {type: [Number], required:true}, // DataAccountpubkey Id
});

module.exports = model('Tickets', ticketSchema);