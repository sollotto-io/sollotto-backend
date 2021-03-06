const Raffle = require("../models/Raffle");
const protectedResolvers = require("./utils");
const raffleMutations = {
  Mutations: {
    async addRaffle(
      _,
      {
        raffleInput: {
          raffleName,
          urlSlug,
          raffleImage,
          sollotoBranding,
          testingWA,
          liveWA,
          operatorWa,
          vanityUrl,
          raffleStatus,
        },
      },
      context,
      info
    ) {
      try {
        const newRaffle = new Raffle({
          raffleName,
          urlSlug,
          raffleImage,
          sollotoBranding,
          testingWA,
          liveWA,
          operatorWa,
          vanityUrl,
          raffleStatus,
        });

        await newRaffle.save();
        return "Raffle Added Successfully";
      } catch (e) {
        console.log(e);
      }
    },
    async changeRaffleStatus(_, { raffleId, Status }, context, info) {
      await Raffle.findByIdAndUpdate(raffleId, { Status });
      return "Raffle Status Updated";
    },

    async editRaffle(parent, { raffleId, raffleInput }, context, info) {
      const {
        raffleName,
        urlSlug,
        raffleImage,
        sollotoBranding,
        testingWA,
        liveWA,
        operatorWa,
        vanityUrl,
        raffleStatus,
      } = raffleInput;

      await Raffle.findByIdAndUpdate(raffleId, {
        raffleName,
        urlSlug,
        raffleImage,
        sollotoBranding,
        testingWA,
        liveWA,
        operatorWa,
        vanityUrl,
        raffleStatus,
      });
      return "Raffle Status Updated";
    },
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
    async getActiveRaffle(_, args, context, info) {
      try {
        const raffle = await Raffle.find({ Status: true });
        return raffle;
      } catch (e) {
        console.log(e);
      }
    },
  },
};

module.exports = {
  Query: raffleMutations.Query,
  Mutations: protectedResolvers(raffleMutations.Mutations),
};
