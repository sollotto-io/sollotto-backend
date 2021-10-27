const NFT = require("../models/Nft");
const protectedResolvers = require("./utils");

const nftQueries = {
  Mutations: {
    async addNFT(_, { nftInput }, context, info) {
      const newNFT = new NFT({ ...nftInput });
      await newNFT.save();
      return newNFT;
    },
    async updateNFt(_, { nftId, nftInput }, context, info) {
      await NFT.findByIdAndUpdate(nftId, { ...nftInput });

      const Nft = NFT.findById(nftId);
      return Nft;
    },

    async addNftTicket(
      _,
      { walletId, dataAccountId, transactionId },
      context,
      info
    ) {
      const nftLotteries = await NFT.find({ status: "live" });
      const nftLottery = nftLotteries[0];
      try {
        if (nftLottery) {
          nftLottery.tickets.push({
            walletId,
            dataAccountId,
            transactionId,
          });

          await nftLottery.save();
          return {
            walletId,
            dataAccountId,
            transactionId,
          };
        } else {
          throw new Error("Cannot register ticket");
        }
        return "Ticket successfully registered";
      } catch (e) {
        throw new Error("Cannot register ticket");
      }
    },
  },
  Query: {
    async getAllNfts() {
      const nftsLotteries = await NFT.find();
      return nftsLotteries;
    },
    async getActiveNft() {
      const nftLottery = await NFT.find({ status: "live" });
      return nftLottery[0];
    },
  },
};

module.exports = {
  Query: nftQueries.Query,
  Mutation: nftQueries.Mutations,
};
