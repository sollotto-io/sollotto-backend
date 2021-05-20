const { model, Schema } = require('mongoose');

const ticketSchema = new Schema({
   walletID: String,
   ticketArray: [String],
   charityName: String
});

module.exports = model('Tickets', ticketSchema);