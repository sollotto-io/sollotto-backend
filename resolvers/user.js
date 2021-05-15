const User = require("../models/user");

module.exports = {
  Mutation: {
    async addUser(_, { walletId }, context, info) {             // wallet id that user will enter
      const newUser = new User({                                //create a new user object to save
        walletId,
      });

      const res = await newUser.save();                         //save the user obj    
      return {
        ...res._doc,
        id: res._id,
      };
    },

    async removeUser(_, { userId }, context, info) {              //userId that will be saved in local storage
      await User.findByIdAndRemove(userId);                       //find the id in db and remove it
      return "User is successfully removed";
    },
  },
};
