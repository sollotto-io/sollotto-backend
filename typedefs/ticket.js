const gql = require("graphql-tag");

module.exports = gql`
  type Ticket {
    id: ID!
    walletID: String!
    ticketArray:[String!]
    charityName:String!
  }

  extend type Mutation {
      addTicket(walletID:String!, ticketArray:[String]!, charityName:String!): Ticket! 
  }

`