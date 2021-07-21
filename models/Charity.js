const { model, Schema } = require('mongoose');

const charitySchema = new Schema({
    charityId:{type:Number},
    charityName: String,
    projectDetails: String,
    ImageURL:String,
    fundUse:String,
    currentVotes: {type:Number},
    addedBy: String,
    lifeTimeVotes: {type:Number},    
    lifeTimeWins: {type:Number},    
    Status:Boolean,
    Years: String,
    URL:String,
    isWatch:Boolean,
    Grade: String,
    Impact:String,
    webURL: String,
    socialMedia:String,
    nominationVotes:{type:Number},
    lifeTimeNominationVotes:{type:Number},
    LastNominationVote:String,
    publicKey:String
});

module.exports = model('Charity', charitySchema);
