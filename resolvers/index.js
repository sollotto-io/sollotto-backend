const charityResolvers = require("./Charity");
const ticketResolvers = require("./Ticket");
const lotteryResolvers = require("./Lottery");
const drawingResolvers = require("./Drawing");
const userResolvers = require("./User");
const raffleResolvers = require("./Raffle");
const LaunchPadResolvers = require("./LaunchPad");
const poolResolvers = require("./Pool");
const adminUserResolvers = require("./adminUsers");
const Model4 = require("./Model4");
const NFT = require("./Nft");
module.exports = {
  Mutation: {
    ...charityResolvers.Mutation,
    ...ticketResolvers.Mutation,
    ...drawingResolvers.Mutation,
    ...raffleResolvers.Mutations,
    ...LaunchPadResolvers.Mutations,
    ...poolResolvers.Mutations,
    ...adminUserResolvers.Mutations,
    ...NFT.Mutation,
  },
  Query: {
    ...charityResolvers.Query,
    ...drawingResolvers.Query,
    ...ticketResolvers.Query,
    ...lotteryResolvers.Query,
    ...userResolvers.Query,
    ...raffleResolvers.Query,
    ...LaunchPadResolvers.Query,
    ...poolResolvers.Query,
    ...adminUserResolvers.Query,
    ...Model4.Query,
    ...NFT.Query,
  },
};
