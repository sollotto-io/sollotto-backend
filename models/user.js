const { model, Schema } = require('mongoose');

const userSchema = new Schema({
  walletId: String
});

module.exports = model('User', userSchema);