const gql = require("graphql-tag");

module.exports = gql`
  type Ticket {
    id: ID!
    walletID: [Int]!
    ticketArray:[Int!]
    DataWallet : [Int!]
    charityName:String!
  }

  input ticketInput {
    walletID: [Int]!
    ticketArray:[Int!]
    DataWallet : [Int!]
    charityName:String!
  }

  extend type Mutation {
      addTicket(ticketInput: ticketInput): Ticket! 
  }

`