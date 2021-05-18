const { model, Schema } = require('mongoose');

const charitySchema = new Schema({
    charityName: String,
    projectDetails: String,
    // currentVotes: Number,
    addedBy: String,
    // lifeTimeVotes: Number,    
    Status:String
});

module.exports = model('Charity', charitySchema);