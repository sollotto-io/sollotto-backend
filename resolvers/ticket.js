const Ticket = require("../models/ticket");

module.exports = {
  Mutation: {
    async addTicket(_, { walletID, ticketArray, charityName }, context, info) {             // wallet id that user will enter
      const newTicket = new Ticket({                                //create a new user object to save
        walletID,
        ticketArray,
        charityName
      });

      const res = await newTicket.save();                         //save the user obj    
      return {
        ...res._doc,
        id: res._id,
      };
    },

  },
};
