
const charityResolvers = require("./charity")
const ticketResolvers = require('./ticket')
const poolResolvers = require('./pool')
const lotteryResolvers = require('./lottery')
module.exports = {
   
    Mutation: {
        
        ...charityResolvers.Mutation,
        ...ticketResolvers.Mutation,
        ...poolResolvers.Mutation,
        ...lotteryResolvers.Mutations

    },
    Query:{
        ...charityResolvers.Query,
        ...poolResolvers.Query,
        ...ticketResolvers.Query,
        ...lotteryResolvers.Query
    }

}