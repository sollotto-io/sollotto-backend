const { UserInputError } = require("apollo-server-express");
const moment = require("moment");
const User = require("../models/User");

module.exports = {
  Query: {
    async getSingleUser(_, { UserPK }, context, info) {
      try {
        const user = await User.findOne({ UserPK: UserPK });
        return user;
      } catch (err) {
        throw new Error(err);
      }
    },
  },
};
