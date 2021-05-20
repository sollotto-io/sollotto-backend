
const charityResolvers = require("./charity")
const ticketResolvers = require('./ticket')
module.exports = {
   
    Mutation: {
        
        ...charityResolvers.Mutation,
        ...ticketResolvers.Mutation

    },
    Query:{
        ...charityResolvers.Query
    }

}