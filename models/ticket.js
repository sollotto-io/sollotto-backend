const { model, Schema } = require('mongoose');

const ticketSchema = new Schema({
   walletID:  {type: [Number], required:true}, //walletpk
   ticketArray: {type: [Number], required:true},
   charityName: String,
   DataWallet: {type: [Number], required:true}, // DataAccountpubkey Id
});

module.exports = model('Tickets', ticketSchema);