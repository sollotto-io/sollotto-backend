const gql = require("graphql-tag");

module.exports = gql`
  type Ticket {
    id: ID!
    walletID: [Int]!
    ticketArray: [Int!]
    DataWallet: [Int!]
    charityId: Int!
  }

  input ticketInput {
    walletID: [Int]!
    ticketArray: [Int!]
    DataWallet: [Int!]
    charityId: Int!

  }

  extend type Mutation {
    addTicket(
      walletID: [Int]!
      ticketArray: [Int!]
      DataWallet: [Int!]
      charityId: Int!
    ): Ticket!
  }
`;
