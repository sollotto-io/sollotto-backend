
const charityResolvers = require("./charity")
const ticketResolvers = require('./ticket')
const lotteryResolvers = require('./lottery')
const drawingResolvers = require('./drawing')
module.exports = {
   
    Mutation: {
        
        ...charityResolvers.Mutation,
        ...ticketResolvers.Mutation,
      ...drawingResolvers.Mutation,
        ...lotteryResolvers.Mutations

    },
    Query:{
        ...charityResolvers.Query,
        ...drawingResolvers.Query,
        // ...ticketResolvers.Query,
        ...lotteryResolvers.Query
    }

}