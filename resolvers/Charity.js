const Charity = require("../models/Charity");
const User = require('../models/User')
const { CHARITY_STATUS } = require("../config");
module.exports = {
  Mutation: {
    async addCharity(
      _,
      {
        charityInput: {
        
          charityName,
          projectDetails,
          fundUse,
          addedBy,
          Status,
          Years,
          URL,
          isWatch,
          Grade,
          Impact,
          webURL,
          socialMedia
        },
      },
      context,
      info
    ) {
      const newCharity = new Charity({
    
        charityName,
        projectDetails,
        fundUse,
        addedBy,
        Status,
        Years,
        URL,
        isWatch,
        Grade,
        Impact,
        webURL,
        socialMedia
      });
      const res = await newCharity.save();
      return {
        ...res._doc,
        id: res._id,
      };
    },
    async addNominationVotes(_,{CharityId, UserPk,Votes},context,info){
        await Charity.findByIdAndUpdate(CharityId,{$inc:{nominationVotes:Votes}});
        await User.findOneAndUpdate({UserPK:UserPk},{$inc:{TokenValue:-Votes}})
      return "Votes added successfully"
    }
  },
  Query: {
    async getAllCharities(_, args, context, info) {
      try {
        const charities = await Charity.find().sort({ createdAt: -1 });
        return charities;
      } catch (err) {
        throw new Error(err);
      }
    },
    async getActiveCharities(_, args, context, info) {
      try {
        const charities = await Charity.find({Status:"Active"}).sort({ createdAt: -1 });
       
        return charities;
      } catch (err) {
        throw new Error(err);
      }
    },
  },
};
