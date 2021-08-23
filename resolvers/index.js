const charityResolvers = require("./Charity");
const ticketResolvers = require("./Ticket");
const lotteryResolvers = require("./Lottery");
const drawingResolvers = require("./Drawing");
const userResolvers = require('./User')
const raffleResolvers = require('./Raffle')
const LaunchPadResolvers = require('./LaunchPad')
const poolResolvers = require('./Pool')
module.exports = {
	Mutation: {
		...charityResolvers.Mutation,
		...ticketResolvers.Mutation,
		...drawingResolvers.Mutation,
		...raffleResolvers.Mutations,
		...LaunchPadResolvers.Mutation,
		...poolResolvers.Mutations
	},
	Query: {
		...charityResolvers.Query,
		...drawingResolvers.Query,
		// ...ticketResolvers.Query,
		...lotteryResolvers.Query,
		...userResolvers.Query,
		...raffleResolvers.Query,
		...LaunchPadResolvers.Query,
		...poolResolvers.Query
	},
};
