const Ticket = require("../models/Ticket");
const Drawing  = require("../models/Drawing");
const Charity  = require("../models/Charity")
module.exports = {
  Mutation: {
    async addTicket(_, { walletID,
      ticketArray,
      DataWallet,
      charityId,
      drawingId,TransactionId},context, info){
       
        const newTicket = new Ticket({
          walletID,
          ticketArray,
          charityId,
          DataWallet,
          drawingId,
          TransactionId
        });
        console.log(newTicket)
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

 await Charity.findByIdAndUpdate(charityId,{  $inc: {
          currentVotes: 1,
          lifeTimeVotes:1
        }, },{new:true})
        return "Ticket Saved Successfully";


      }
  },
  Query: {
  
  },
};
