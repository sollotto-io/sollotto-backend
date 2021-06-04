const Ticket = require("../models/ticket");

module.exports = {
  Mutation: {
    async addTicket(_,{   walletID, ticketArray, charityId ,DataWallet }, context, info) {       
      

      
      const newTicket = new Ticket({                                              
        walletID,
        ticketArray,
        charityId,
        DataWallet
      });

      const res = await newTicket.save();                            
      return "Ticket Saved Successfully"
    },

  },
};
