const { model, Schema } = require('mongoose');

const ticketSchema = new Schema({
   walletID: String,
   ticketArray: {type: [Number], required:true},
   charityName: String
});

module.exports = model('Tickets', ticketSchema);