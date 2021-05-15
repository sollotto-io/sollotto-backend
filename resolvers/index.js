const userResolvers = require('./user');
module.exports = {
   
    Mutation: {
        ...userResolvers.Mutation,

    }
}