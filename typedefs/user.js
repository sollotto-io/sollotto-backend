const gql = require("graphql-tag");

module.exports = gql`
  type User {
    id: ID!
    walletId: String!
  }

  extend type Mutation {
      addUser(walletId:String!): User! 
  }

`