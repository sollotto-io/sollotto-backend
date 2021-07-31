const Charity = require("../models/Charity");
const User = require("../models/User");
const { createWriteStream, existsSync, mkdirSync } = require("fs");
const path = require("path");
const _ = require("lodash");
const { CHARITY_STATUS } = require("../config");
const { UserInputError } = require("apollo-server-express");
const { ValidateUpdateProjectInput } = require("../utils/helpers");
module.exports = {
  Mutation: {
    async addCharity(
      _,
      {
        charityInput: {
          charityName,
          projectDetails,
          ImageURL,
          fundUse,
          addedBy,
          Status,
          Years,
          URL,
          isWatch,
          Grade,
          Impact,
          webURL,
          socialMedia,
          publicKey,
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
        ImageURL,
        Status,
        Years,
        URL,
        isWatch,
        Grade,
        Impact,
        webURL,
        socialMedia,
        publicKey,
        nominationVotes: 0,
        lifeTimeNominationVotes:0,
        LastNominationVote: "",
        currentVotes: 0,
        lifeTimeVotes: 0,
        lifeTimeWins: 0,
      });
      const res = await newCharity.save();
      return "Charity Added Successfully"
    },
    async addNominationVotes(_, { charityId, UserPk, Votes }, context, info) {
      await Charity.findByIdAndUpdate(charityId, {
        $inc: { nominationVotes: Votes, lifeTimeNominationVotes: Votes },
        LastNominationVote: Date.now().toString(),
      });
      await User.findOneAndUpdate(
        { UserPK: UserPk },
        { $inc: { TokenValue: -Votes } }
      );

      return "Votes added successfully";
    },
    async deleteCharity(parent, { charityId, Status }, context, info) {
      await Charity.findByIdAndUpdate(charityId, { Status: Status });

      return "Charity Status Updated";
    },
    async updateCharity(parent, { charityId, charityInput }, context, info) {
      const { data, isValid, errors } =
        ValidateUpdateProjectInput(charityInput);
      if (!isValid) throw new UserInputError("Errors", { errors });
      try {
        const updatedCharity = await Charity.findByIdAndUpdate(
          charityId,
          {  charityName: data.charityName,
            projectDetails: data.projectDetails,
            ImageURL: data.ImageURL,
            fundUse: data.fundUse,
            addedBy: data.addedBy,
            Status: data.Status,
            Years: data.Years,
            isWatch: data.isWatch,
            URL: data.URL,
            Grade: data.Grade,
            Impact: data.Impact,
            webURL: data.webURL,
            socialMedia: data.socialMedia,
            publicKey: data.publicKey },
          { new: true }
        );
        if (updatedCharity) return "Charity Updated Sucessfully";
      } catch (e) {
        console.log(e);
      }
      throw new UserInputError("cannot update charity");
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
  },
};
