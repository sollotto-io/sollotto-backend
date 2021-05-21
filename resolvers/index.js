
const charityResolvers = require("./charity")
const ticketResolvers = require('./ticket')
const poolResolvers = require('./pool')
module.exports = {
   
    Mutation: {
        
        ...charityResolvers.Mutation,
        ...ticketResolvers.Mutation,
        ...poolResolvers.Mutation

    },
    Query:{
        ...charityResolvers.Query,
        ...poolResolvers.Query,
    }

}