const { model, Schema } = require('mongoose');

const userSchema = new Schema({
    charityName: String,
    projectDetails: String,
    currentVotes: Int,
    addedBy: String,
    lifeTimeVotes: Int,    
    Status:String
});

module.exports = model('Charity', charitySchema);