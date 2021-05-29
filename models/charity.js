const { model, Schema } = require('mongoose');

const charitySchema = new Schema({
    charityName: String,
    projectDetails: String,
    currentVotes: {type:Number},
    addedBy: String,
    lifeTimeVotes: {type:Number},    
    lifeTimeWins: {type:Number},    
    Status:String
});

module.exports = model('Charity', charitySchema);