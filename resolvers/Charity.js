const Charity = require("../models/Charity");
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
          watchURL,
          watchGrade,
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
        watchURL,
        watchGrade,
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
        const charities = await Charity.find().sort({ createdAt: -1 });
        activeCharities = charities.filter(
          (p) => p.Status === CHARITY_STATUS.VOTE_NOW
        );
        return activeCharities;
      } catch (err) {
        throw new Error(err);
      }
    },
  },
};
