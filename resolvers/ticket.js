const Ticket = require("../models/ticket");

module.exports = {
  Mutation: {
    async addTicket(_,{  ticketInput: { walletID, ticketArray, charityName ,DataWallet }}, context, info) {       
      

      
      const newTicket = new Ticket({                                              
        walletID,
        ticketArray,
        charityName,
        DataWallet
      });

      const res = await newTicket.save();                            
      return {
        ...res._doc,
        id: res._id,
      };
    },

  },
};
