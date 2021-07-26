const Raffle = require("../models/Raffle");

module.exports = {
  Mutations: {
    async addRaffle(
      _,
      {
        raffleInput: {
          raffleName,
          publicKey,
          ImageURL,
        },
      },
      context,
      info
    ) {
      try {
        const newRaffle = new Raffle({
          raffleName,
          publicKey,
          ImageURL,
          Status: false
        });

        await newRaffle.save();
        return "Raffle Added Successfully"
      } catch (e) {
        console.log(e);
      }
    },
    async changeRaffleStatus(_,{raffleId, Status}, context, info){

        await Raffle.findByIdAndUpdate(raffleId, {Status, Status});
        return "Raffle Status Updated"
    }
  },
  Query: {
    async getAllRaffle(_, args, context, info) {
      try {
        const raffle = await Raffle.find().sort({ createdAt: -1 });
        return raffle;
      } catch (err) {
        throw new Error(err);
      }
    },
    async getActiveRaffle(_, args, context, info){
        try{
            const raffle = await Raffle.findOne({Status:true})
            return raffle;
        }catch(e){
            console.log(e)
        }
    }
  },
};
