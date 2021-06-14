const Ticket = require("../models/Ticket");
const Drawing  = require("../models/Drawing");
const Lottery  = require("../models/Lottery");
module.exports = {
  Mutation: {
    async addTicket(_, { walletID,
      ticketArray,
      DataWallet,
      charityId,
      drawingId},context, info){
       
        const newTicket = new Ticket({
          walletID,
          ticketArray,
          charityId,
          DataWallet,
          drawingId,
        });
        const res = await newTicket.save();
        await Drawing.findOne({ _id: drawingId });
        await Drawing.findOneAndUpdate(
          { _id: drawingId, "CharityVoteCount.charityId": charityId },
          {
            $push:{
Tickets: res._id
            },
            $inc: {
              TotalPoolValue: 0.1,
              TotalRegistrations: 1,
              "CharityVoteCount.$.votes": 1,
            },
          },
          { new: true }
        );
        
        return "Ticket Saved Successfully";


      }
  },
  Query: {
  
  },
};
