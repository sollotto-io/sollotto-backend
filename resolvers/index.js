const userResolvers = require('./user');
const charityResolvers = require("./charity")
module.exports = {
   
    Mutation: {
        ...userResolvers.Mutation,
        ...charityResolvers.Mutation,
        

    },
    Query:{
        ...charityResolvers.Query
    }

}