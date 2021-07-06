const charityResolvers = require("./Charity");
const ticketResolvers = require("./Ticket");
const lotteryResolvers = require("./Lottery");
const drawingResolvers = require("./Drawing");
const userResolvers = require('./User')
module.exports = {
	Mutation: {
		...charityResolvers.Mutation,
		...ticketResolvers.Mutation,
		...drawingResolvers.Mutation,
		...lotteryResolvers.Mutations,
	},
	Query: {
		...charityResolvers.Query,
		...drawingResolvers.Query,
		// ...ticketResolvers.Query,
		...lotteryResolvers.Query,
		...userResolvers.Query

	},
};
