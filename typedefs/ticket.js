const gql = require("graphql-tag");

module.exports = gql`
  type Ticket {
    id: ID!
    walletID: String!
    ticketArray:[Int!]
    charityName:String!
  }

  extend type Mutation {
      addTicket(walletID:String!, ticketArray:[Int]!, charityName:String!): Ticket! 
  }

`